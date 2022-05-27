
# Copyright © 2021 - 2022 HQS Quantum Simulations GmbH.
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

from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import PauliZProduct, PauliZProductInput, ClassicalRegister
from qoqo import QuantumProgram
import tempfile
from pathlib import Path
import json

# This example demonstrates how to serialize a QuantumProgram to json.
# It also prints and writes the example jsons to file.
# Please note that the changes to the json format are going to be made for qoqo 1.0
# This example gives the serialisation of two QuantumPrograms:
# * A quantum program based on a pauli product measurement that returns expectation values when run
# * A register readout quantum program that returns the measurement results without postprocessing when run

# Creating a program with an initialisation Circuit and two readout-circuits.
init_circuit = Circuit()
# Apply a RotateY gate with a symbolic angle
# To execute the circuit this symbolic parameter needs to be replaced 
# with a real number with the help of a QuantumProgram
init_circuit += ops.RotateX(0, "angle")
# Z-basis measurement circuit with 1000 shots
z_circuit = Circuit()
z_circuit += ops.DefinitionBit("ro_z", 1, is_output=True)
z_circuit += ops.PragmaRepeatedMeasurement("ro_z", 1000, None)
# X-basis measurement circuit with 1000 shots   
x_circuit = Circuit()
x_circuit += ops.DefinitionBit("ro_x", 1, is_output=True)
# Changing to the X basis with a Hadamard gate
x_circuit += ops.Hadamard(0)
x_circuit += ops.PragmaRepeatedMeasurement("ro_x", 1000, None)

# Preparing the measurement input for one qubit
measurement_input = PauliZProductInput(1, False)
# Read out product of Z on site 0 for register ro_z (no basis change)
z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
# Read out product of Z on site 0 for register ro_x (after basis change effectively a <X> measurement)
x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])

# Add a result (the expectation value of H) that is a combination of the PauliProduct expectation values
measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

pauli_product_measurement = PauliZProduct(constant_circuit=init_circuit, circuits=[z_circuit, x_circuit], input=measurement_input)
register_measurement = ClassicalRegister(constant_circuit=init_circuit, circuits=[z_circuit, x_circuit])

# Serialisation of Program returning register measurements with postprocessing
program = QuantumProgram(measurement=register_measurement, input_parameter_names=["angle"])
program_json = program.to_json()
# Trick to make the json easier to read
json_dict = json.loads(program_json)
json_pretty_string = json.dumps(json_dict, indent = 2, separators=(',', ': '))
# Print the json representation
print("Json for register measurement")
print(json_pretty_string)

# Getting tmp file path for example
tmp = Path(tempfile.gettempdir())
# print(tmp)
# writing json to file
with  open(tmp / "serialised_quantum_program_measuring_registers.json", "w") as f:
    f.write(json_pretty_string)


# Serialisation of Program returning expectation values
program = QuantumProgram(measurement=pauli_product_measurement, input_parameter_names=["angle"])
program_json = program.to_json()
# Trick to make the json easier to read
json_dict = json.loads(program_json)
json_pretty_string = json.dumps(json_dict, indent = 2, separators=(',', ': '))
# Print the json representation
#print("Json for pauli product measurement")
print(json_pretty_string)

# Getting tmp file path for example
tmp = Path(tempfile.gettempdir())
#print(tmp)
# writing json to file
with  open(tmp / "serialised_quantum_program_measuring_expectation_values.json", "w") as f:
    f.write(json_pretty_string)