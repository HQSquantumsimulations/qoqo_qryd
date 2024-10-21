# Copyright © 2024 HQS Quantum Simulations GmbH.

"""Utils for TweezerMutableDevice layout setting."""

from typing import Optional, Tuple, List


def row_column_to_index(row: int, column: int, columns: int) -> int:
    """Returns index for a given row and column."""
    return row * (columns) + column


def apply_column_square(
    row: int, column: int, columns: int, rows: int
) -> Optional[Tuple[int, int]]:
    """Return indices for interaction along y-direction in a square lattice.

    Returns:
        (int, int): (origin_index, target_index)
    """
    if row < rows - 1:
        origin = row_column_to_index(row, column, columns)
        target = row_column_to_index(row + 1, column, columns)
        return (origin, target)
    else:
        return None


def apply_column_triangular(
    row: int, column: int, columns: int, rows: int
) -> Optional[List[Tuple[int, int]]]:
    """Return indices for a down leaning interaction in a square lattice.

    Returns:
        (int, int): (origin_index, target_index)
    """
    vertical_connections = []
    if row < rows - 1:
        # Normal vertical edge
        origin = row_column_to_index(row, column, columns)
        target = row_column_to_index(row + 1, column, columns)
        vertical_connections.append((origin, target))
        # Additional vertical edge, depending on row parity
        if row % 2 == 0 and column < columns - 1:
            target = row_column_to_index(row + 1, column + 1, columns)
            vertical_connections.append((origin, target))
        elif row % 2 == 1 and column > 0:
            target = row_column_to_index(row + 1, column - 1, columns)
            vertical_connections.append((origin, target))
        return vertical_connections
    else:
        return None


def apply_row(
    row: int, column: int, columns: int, _rows: int
) -> Optional[Tuple[int, int]]:
    """Return indices for interaction along x-direction in a square lattice.

    Returns:
        (int, int): (origin_index, target_index)
    """
    if column < columns - 1:
        return (
            row_column_to_index(row, column, columns),
            row_column_to_index(row, column + 1, columns),
        )
