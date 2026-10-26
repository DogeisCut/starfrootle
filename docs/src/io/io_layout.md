# I/O Layout

> [!NOTE]
> This page is currently a stub and needs more detail.

I/O starts at `0x420000`

| Offset  | Size         | Contents                              |
|---------|--------------|---------------------------------------|
| `+0x00` | `0x78`       | [Controller table](#controller-table) |


## Controller Table 

8 [controller entries](./input.md#controller-entry) of 16 bytes each, for a total of 128 bytes. Controller N lives 
at `i/o start + N * 16`.

