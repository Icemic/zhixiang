# Asset Bundle Format Specification

## File Header (13 bytes)

| Offset | Size | Description        | Note     |
| ------ | ---- | ------------------ | -------- |
| 0      | 5    | Magic Header       | "NEKO\0" |
| 5      | 1    | Major Version      |          |
| 6      | 1    | Minor Version      |          |
| 7      | 1    | Patch Version      |          |
| 8      | 1    | Flags              | Reserved |
| 9      | 4    | Index Table Offset |          |

## Index Table

| Offset | Size | Description   | Note      |
| ------ | ---- | ------------- | --------- |
| 0      | 4    | Entry Count   |           |
| 4      | \*   | Index Entries | See below |

## Index Entry Format

| Offset | Size | Description  | Note      |
| ------ | ---- | ------------ | --------- |
| 0      | 1    | Asset Type   | See below |
| 1      | 2    | Path Length  |           |
| 3      | N    | Path String  | UTF-8     |
| 3+N    | 8    | Data Start   |           |
| 11+N   | 8    | Data Size    |           |
| 19+N   | 2    | Extra Size   |           |
| 21+N   | M    | Extra Fields | Reserved  |

## Notes

- All integer values are stored in little-endian format
- Path strings are UTF-8 encoded
- Data offset is relative to the position of the offset field
