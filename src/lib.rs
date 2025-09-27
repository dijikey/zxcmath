/*
*   __     __  _____    ____   _____    ___    ____
*   \ \   / / | ____|  / ___| |_   _|  / _ \  |  _ \
*    \ \ / /  |  _|   | |       | |   | | | | | |_) |
*     \ V /   | |___  | |___    | |   | |_| | |  _ <
*      \_/    |_____|  \____|   |_|    \___/  |_| \_\
*/
mod vector3;

#[derive(Debug, Clone, Copy)]
struct Vector3<T>
where T: Clone + Copy {
    pub x: T,
    pub y: T,
    pub z: T,
}