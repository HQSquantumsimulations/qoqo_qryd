"""A simple example for shifting the qubit positions."""

# Copyright © 2021-2022 HQS Quantum Simulations GmbH.

import numpy as np
from qoqo import Circuit
import qoqo.operations as ops
from qoqo_qryd import devices, SimulatorBackend
from qoqo_qryd import pragma_operations as qrydops


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
device = devices.FirstDevice(
    number_rows=2,
    number_columns=4,
    qubits_per_row=[2, 2],
    row_distance=1.0,
    initial_layout=np.array([
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0]
    ])
)

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
# Setting up the device in the square lattice
device.change_qubit_positions({0: (0, 0), 1: (0, 1), 2: (1, 0), 3: (1, 1)})
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
circuit += qrydops.PragmaShiftQRydQubit(
    {0: (0, 0), 1: (0, 1), 2: (1, 1), 3: (1, 2)}).to_pragma_change_device()
circuit += ops.PhaseShiftedControlledZ(control=1, target=2, phi=0.0)
circuit += ops.PragmaGetStateVector("state_vector_before", None)
# This should pass
result = backend.run_circuit(circuit)
print("State vector before applying shift and two-qubit gate")
print(result[2]["state_vector_before"])
print("State vector after applying shift and two-qubit gate")
print(result[2]["state_vector_after"])
