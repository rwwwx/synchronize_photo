# synchronize_photo
Synchronize Photo Collections by Date

# To run this you need
1. Clone repository `git clone git@github.com:rwwwx/synchronize_photo.git` 
2. To run use: `cargo r -- /full/path/to/your/photos` for example: `cargo r -- /Users/vladyslav.matiukhin/Documents/photo_example`   
3. Or use `cargo r -- ` to run this program on `photo_example` folder inside of project.

# MAKE SURE
Make sure your folder structure looks like this:

    /// 2024-04-15 -> Lev   -> Photo1, Photo2, Photo3
    ///            -> My    -> Photo1, Photo5, Photo3
    ///            -> Denis -> Photo1, Photo2, Photo4
    ///
    /// 2024-04-16 -> Lev   -> Photo1, Photo2, Photo3
    ///            -> My    -> Photo1, Photo5, Photo3
    ///            -> Denis -> Photo1, Photo2, Photo4

1. Strictly follow the form of writing the date when naming day folders.
2. Your folder should always be named `My`. 
3. Even if no photo was taken on the day the folder should still exist. 