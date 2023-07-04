use std::collections::{HashSet, VecDeque};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use protobuf::{descriptor::FileDescriptorSet, Message, MessageFull};

fn get_fd_set<T: MessageFull>() -> FileDescriptorSet {
    let mut fd_set = FileDescriptorSet::new();
    let mut to_add = VecDeque::new();
    let mut added = HashSet::<String>::new();
    to_add.push_back(T::descriptor().file_descriptor().clone());

    while let Some(next) = to_add.pop_front() {
        for dep in next.deps() {
            if !added.contains(dep.name()) {
                added.insert(dep.name().into());
                to_add.push_back(dep.clone());
            }
        }
        fd_set.file.push(next.proto().clone());
    }

    fd_set
}

pub fn base64_proto<T: MessageFull>() -> String {
    let buf = get_fd_set::<T>().write_to_bytes().unwrap();

    STANDARD.encode(buf)
}

#[cfg(test)]
mod test {
    use digitalis_protobuf::{Color, Pose};

    use super::*;

    #[test]
    fn test_fd_set() {
        let fd_set = get_fd_set::<Color>();

        assert_eq!(fd_set.file.len(), 1);
        assert_eq!(
            fd_set.file[0].name.as_ref().unwrap(),
            "foxglove/Color.proto"
        );

        let fd_set = get_fd_set::<Pose>();

        assert_eq!(fd_set.file.len(), 3);

        for msg in &["Vector3", "Quaternion", "Pose"] {
            assert!(fd_set
                .file
                .iter()
                .any(|f| f.name == Some(format!("foxglove/{}.proto", msg))));
        }
    }
}
