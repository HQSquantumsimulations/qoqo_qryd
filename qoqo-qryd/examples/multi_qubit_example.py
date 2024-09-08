"""A simple example demonstrating multi qubit operations.

Kept as past reference, as there are no multi-qubit operations that are natively
supported by TweezerDevice and TweezerMutableDevice."""

# Copyright © 2021 - 2024 HQS Quantum Simulations GmbH.
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
from qoqo import Circuit
import qoqo.operations as ops
from qoqo_qryd import qryd_devices, SimulatorBackend
from qoqo_qryd import pragma_operations as qrydops

# --------------------- The set-up of the device ----------------------- #

# Initializing Device with a square lattice
#
#   Tweezer positions:
#   (0, 0) ----- (0, 1) ----- (0, 2) ----- (0, 3)
#   (1, 0) ----- (1, 1) ----- (1, 2) ----- (1, 3)
#   (2, 0) ----- (2, 1) ----- (2, 2) ----- (2, 3)
#
#   Qubit positions:
#   0 --- 1 --- 2 --- 3
#   4 --- 5 --- 6 --- 7
#   8 --- 9 --- 10 --- 11
#
device = qryd_devices.FirstDevice(
    number_rows=3,
    number_columns=4,
    qubits_per_row=[4, 4, 4],
    row_distance=1.0,
    initial_layout=np.array(
        [[0.0, 1.0, 2.0, 3.0], [0.0, 1.0, 2.0, 3.0], [0.0, 1.0, 2.0, 3.0]]
    ),
)

backend = SimulatorBackend(device)

# ---------------- Multi Qubit Circuits that will fail ---------------------- #

# For the Prototype we assume that only MultiQubitZZ operations
# are allowed between qubits in one row.
# This is an arbitrary limitation implemented to showcase
# how a restricted operation would be implemented
# in the final device.

# Use a MultiQubitMS Molemer-Sorensen Gate
circuit = Circuit()
# MultiQubitMS not supported natively
circuit += ops.MultiQubitMS(qubits=[0, 1, 2, 3], theta=1.0)
# This should fail
# result = backend.run_circuit(circuit)

# Use a MultiQubitZZ  Gate but in the wrong direction
circuit = Circuit()
# MultiQubitZZ not supported along a column
circuit += ops.MultiQubitZZ(qubits=[0, 4, 8], theta=1.0)
# This should fail
# result = backend.run_circuit(circuit)


# --------------------- Working Multi Qubit Circuit ------------------------- #

circuit = Circuit()
circuit += ops.DefinitionBit("ro", 12, True)
circuit += ops.MultiQubitZZ(qubits=[0, 1, 2, 3], theta=1.0)
circuit += ops.PragmaRepeatedMeasurement("ro", 1, None)
# This should pass
result = backend.run_circuit(circuit)
print("Result of ZZ on first four qubits")
print(result)
