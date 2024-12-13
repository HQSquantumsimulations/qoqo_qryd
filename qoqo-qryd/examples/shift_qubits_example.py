"""A simple example for shifting the qubit positions."""

# Copyright © 2021-2025 HQS Quantum Simulations GmbH.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.

import numpy as np
import qoqo.operations as ops  # type:ignore
from qoqo import Circuit
from qoqo_qryd import SimulatorBackend
from qoqo_qryd import pragma_operations as qrydops
from qoqo_qryd.tweezer_devices import TweezerMutableDevice  # type:ignore
from utils import apply_column_square, apply_row

# ------------------------- The set-up of the device -------------------------- #

# Initializing Device with a square lattice
#
#   Tweezer positions:
#   (0, 0) ----- (0, 1) ----- (0, 2) ----- (0, 3)
#   (1, 0) ----- (1, 1) ----- (1, 2) ----- (1, 3)
#
#   Qubit positions:
#   0 --- 1
#   2 --- 3
#
rows = 2
columns = 4

device = TweezerMutableDevice()
device.add_layout("square_lattice")

for i in range(rows * columns):
    for gate in ["RotateX", "PhaseShiftState1"]:
        device.set_tweezer_single_qubit_gate_time(gate, i, 1.0, "square_lattice")

for row in range(rows):
    for column in range(columns):
        row_indices = apply_row(row, column, columns, rows)
        column_indices = apply_column_square(row, column, columns, rows)
        if row_indices is not None:
            device.set_tweezer_two_qubit_gate_time(
                "PhaseShiftedControlledZ",
                row_indices[0],
                row_indices[1],
                1.0,
                "square_lattice",
            )
        if column_indices is not None:
            device.set_tweezer_two_qubit_gate_time(
                "PhaseShiftedControlledZ",
                column_indices[0],
                column_indices[1],
                1.0,
                "square_lattice",
            )

device.set_allowed_tweezer_shifts_from_rows(
    [[0, 1, 2, 3], [4, 5, 6, 7]], "square_lattice"
)

device.switch_layout("square_lattice", with_trivial_map=False)

# Populate the device according to initialization explained above
device.add_qubit_tweezer_mapping(0, 0)
device.add_qubit_tweezer_mapping(1, 1)
device.add_qubit_tweezer_mapping(2, 4)
device.add_qubit_tweezer_mapping(3, 5)

backend = SimulatorBackend(device)

# ------------------------ The set-up of the circuit ------------------------ #

circuit = Circuit()
# Qubits 1 and 2 are not close enough for interaction in square lattice
circuit += ops.PhaseShiftedControlledZ(control=1, target=2, phi=0.0)
# This should fail
# result = backend.run_circuit(circuit)


# ------------------ The set-up of the circuit with device change --------------- #

circuit = Circuit()
# Qubits 1 and 2 are close enough for interaction in square lattice after shift
#   Qubit positions after shift:
#   0 --- 1
#         2 --- 3
circuit += ops.DefinitionComplex("state_vector_before", 16, True)
circuit += ops.DefinitionComplex("state_vector_after", 16, True)
circuit += ops.RotateX(1, np.pi)
circuit += ops.RotateX(2, np.pi / 2)
circuit += ops.PragmaGetStateVector("state_vector_before", None)
circuit += qrydops.PragmaShiftQubitsTweezers([(5, 6)]).to_pragma_change_device()
circuit += qrydops.PragmaShiftQubitsTweezers([(4, 5)]).to_pragma_change_device()
circuit += ops.PhaseShiftedControlledZ(control=1, target=2, phi=0.0)
circuit += ops.PragmaGetStateVector("state_vector_after", None)
# This should pass
result = backend.run_circuit(circuit)
print("State vector before applying shift and two-qubit gate")
print(result[2]["state_vector_before"])
print("State vector after applying shift and two-qubit gate")
print(result[2]["state_vector_after"])
