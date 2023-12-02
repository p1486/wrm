#[cfg(test)]
mod tests {
    use crate::{
        actions::{clean, delete, remove, restore},
        FileList, WrmPath,
    };
    use filey::{catenate, create, remove, FileTypes, Filey};
    use std::path::Path;

    #[test]
    fn it_works() {
        let wrm_path = WrmPath::default().expanded().unwrap();
        let f = "test/a.txt";
        let d = "test/a_dir";
        let tf = Filey::new("~/.config/wrm/trash/a.txt")
            .expand_user()
            .unwrap();
        remove!("test", wrm_path.dir());
        create!(
            FileTypes::Directory,
            "test",
            wrm_path.dir(),
            wrm_path.trash(),
            &d,
        );
        create!(FileTypes::File, wrm_path.list(), &f);
        FileList::new().write(wrm_path.list()).unwrap();
        remove(vec![f.to_string()], &wrm_path, true, false).unwrap();
        assert_eq!(Path::new(&f).exists(), false);
        assert_eq!(tf.path().exists(), true);
        println!("{}", catenate!(wrm_path.list()));
        delete(vec![d.to_string()], &wrm_path, true, false).unwrap();
        assert_eq!(Path::new(&d).exists(), false);
        restore(vec![tf.to_string()], &wrm_path, true, false).unwrap();
        assert_eq!(tf.path().exists(), false);
        assert_eq!(Path::new(&f).exists(), true);
        println!("{}", catenate!(wrm_path.list()));
        remove(vec![f.to_string()], &wrm_path, true, false).unwrap();
        clean(&wrm_path, true, false).unwrap();
        assert_eq!(Path::new(wrm_path.trash()).exists(), false);
        assert_eq!(Path::new(wrm_path.list()).exists(), false);
        remove!("test", wrm_path.dir());
    }
}
