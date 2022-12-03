#include "board.hpp"

// extra function to handle exceptions, which are not compatible with the Rust FFI
bool DropDiskToBoardSucceeded(Board &b, DiskType disk, int col) {
  try {
    DropDiskToBoard(b, disk, col);
    return true;
  } catch (...) {
    return false;
  }
}