"""A simple example for switching between a square lattice and a triangular lattice layout."""

# Copyright © 2021-2022 HQS Quantum Simulations GmbH.
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
from qoqo_qryd import devices, SimulatorBackend
from qoqo_qryd import pragma_operations as qrydops


# ------------------------- The set-up of the device -------------------------- #

# Initializing device with a square lattice
#
#   Tweezer positions:
#   (0, 0) ----- (0, 1) ----- (0, 2) ----- (0, 3)
#   (1, 0) ----- (1, 1) ----- (1, 2) ----- (1, 3)
#
#   Qubit positions:
#   0 --- 1
#   2 --- 3
#
device = devices.FirstDevice(
    number_rows=2,
    number_columns=4,
    qubits_per_row=[2, 2],
    row_distance=1.0,
    initial_layout=np.array([
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0]]))
# Adding a triangular lattice
#
#   Tweezer positions:
#   (0, 0) ----- (0, 1) ----- (0, 2) ----- (0, 3)
#          (1, 0) ----- (1, 1) ----- (1, 2) ----- (1, 3)
#
#   Qubit positions:
#   0 --- 1
#      2 --- 3
device = device.add_layout(
    1,
   np.array([
       [0.0, 1.0, 2.0, 3.0],
       [0.5, 1.5, 2.5, 3.5]
   ])
)
# Set the cut-off distance for two-qubit interactions
device.set_cutoff(1.0)
# Setting up the device in the triangular lattice
device.switch_layout(1)
device.change_qubit_positions({0: (0, 0), 1: (0, 1), 2: (1, 0), 3: (1, 1)})
backend = SimulatorBackend(device)

# ----------------------- The set-up of the circuit -------------------------- #

circuit = Circuit()
# Qubits 0 and 2 are not close enough
# for interaction in triangular lattice
circuit += ops.PhaseShiftedControlledZ(control=0, target=2, phi=0.0)
# This should fail
# result = backend.run_circuit(circuit)


# ----------------- The set-up of the circuit with device change --------------- #

circuit = Circuit()
circuit += ops.DefinitionComplex("state_vector_before", 16, True)
circuit += ops.DefinitionComplex("state_vector_after", 16, True)
circuit += ops.RotateX(0, np.pi)
circuit += ops.RotateX(2, np.pi / 2)
circuit += ops.PragmaGetStateVector("state_vector_before", None)
# Qubits 0 and 2 are close enough for interaction in square lattice
circuit += qrydops.PragmaChangeQRydLayout(0).to_pragma_change_device()
circuit += ops.PhaseShiftedControlledZ(control=0, target=2, phi=0.0)
circuit += ops.PragmaGetStateVector("state_vector_after", None)
# This should pass
result = backend.run_circuit(circuit)
print("State vector before applying shift and two-qubit gate")
print(result[2]["state_vector_before"])
print("State vector after applying shift and two-qubit gate")
print(result[2]["state_vector_after"])
