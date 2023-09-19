Introduction
============

The qoqo-qryd package is designed to enable the execution of quantum algorithms implemented in [qoqo](https://github.com/HQSquantumsimulations/qoqo) on QRydDemo hardware.


Low level: quantum circuits
--------------------------

Qoqo is a circuit based quantum computing toolkit. Like many other quantum computing toolkits it can be used to construct quantum circuits - sequences of quantum operations that are to be executed on a quantum computer.

Examples for quantum operations are the controlled NOT (CNOT) operation on two qubits and the Hadamard gate on a single qubit.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   circuit = Circuit()
   # Initializing first qubit in superposition of |0> and |1>
   circuit += ops.Hadamard(0)
   # Entangling qubits 0 and 1 with CNOT
   circuit += ops.CNOT(0,1)
```

To extract information from a quantum computer results must be measured.
For measurements qoqo defines classical registers in the quantum circuits.
The classical measurement results will be written to the classical registers.
The definition of classical registers is similar to a variable declaration in normal programs.

The measurement of a qubit (on hardware) is always a projective measurement in the ``Z``-basis yielding ``0`` or ``1``.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   circuit = Circuit()
   # Defining a bit readout with name "ro" containing 2 bits
   circuit += ops.DefinitionBit("ro", 2, is_output=True)
   # Initializing first qubit in superposition of |0> and |1>
   circuit += ops.Hadamard(0)
   # Entangling qubits 0 and 1 with CNOT
   circuit += ops.CNOT(0,1)
   # Measuring all qubits and writing the results into register 'ro'
   # Repeating the circuit 100 times to create 100 projective measurements
   circuit += ops.PragmaRepeatedMeasurement("ro", 100, None)
```

High-level: quantum programs
----------------------------

On a more abstract level a quantum program can be defined as a program that can be executed on a quantum computer after receiving a list of classical parameters and returns a list of classical results.

Qoqo provides the QuantumProgram class for this purpose. A QuantumProgram is the preferred way to communicate between different programs (for example with hardware or simulators) and to save quantum programs with the qoqo toolkit.

For many applications the measurement results of several circuits need to be combined to extract the required information from a quantum state prepared by the quantum operations in a quantum circuit.
The combination of the results of each quantum circuit happens in a post-processing of classical measurement.

A qoqo measurement combines one ``constant_circuit`` that is always executed first, a list of ``circuits`` that are executed after the constant circuit, and a ``measurement_input`` that encodes the classical post-processing.

As an example take the measurement of a Hamiltonian ``H = 0.1 * X + 0.2 * Z`` where ``X`` and ``Z`` are Pauli operators. We want to measure ``H`` with respect to a state ``|psi> = (|0> + |1>)/sqrt(2)``. 
We will use a Hadamard gate in the ``constant_circuit`` to prepare ``|psi>``. Since we cannot measure ``X`` and ``Z`` at the same time, the ``circuits`` list will include one quantum circuit that does not apply any additional gate and one circuit that rotates the qubit basis into the ``X``-basis so that the expectation value ``<X>`` is equivalent to the measurement of ``<Z>`` in the new basis.
This kind of measurement is referred to as a PauliZProduct measurement because each qubit is rotated in the correct basis for the readout. 
For the post-processing the PauliZProduct measurement needs two more details to be added to the input (``PauliZProductInput``): Which qubits to combine into expectation values (``add_pauliz_product()``) and which weight to use for each result (``add_linear_exp_val()``).

In general one can measure the expectation values of the products of local Z operators, e.g. ``<Z0>``, ``<Z1>``, ``<Z0*Z1>``, ``<Z0*Z3>``, ...
The PauliZProductInput needs to define all of these products that are measured. Here we will measure two products ``<Z0>`` after a rotation in the X basis and ``<Z0>`` without an additional rotation.
The PauliZProductInput also defines the weights of the products in the final result. Here 0.1 for the first product and 0.2 for the second.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   from qoqo.measurements import PauliZProduct, PauliZProductInput
   from qoqo import QuantumProgram
   # Initialize |psi>
   init_circuit = Circuit()
   init_circuit += ops.Hadamard(0)
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
   # The PauliZProductInput starts with just the number of qubits
   # and if to use a flipped measurements set.
   measurement_input = PauliZProductInput(1, False)
   # Next, pauli products are added to the PauliZProductInput
   # Read out product of Z on site 0 for register ro_z (no basis change)
   z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
   # Read out product of Z on site 0 for register ro_x
   # (after basis change effectively a <X> measurement)
   x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])
   
   # Last, instructions on how to combine the single expectation values
   # into the total result are provided.
   # Add a result (the expectation value of H) that is a combination of
   # the PauliProduct expectation values.
   measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

   measurement = PauliZProduct(
      constant_circuit=init_circuit,
      circuits=[z_circuit, x_circuit],
      input=measurement_input,
   )
```

For details on how to execute QuantumPrograms see [Execution](execution.md).


The qoqo QuantumProgram combines a measurement with a list of free parameters that are not set at compilation time but can be dynamically set whenever the QuantumProgram is run.
To demonstrate this we modify the example from above to use a state ``|psi>`` with a free angle between ``|0>`` and ``|1>``. Such a state can be prepared by a ``RotateX`` quantum operation.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   from qoqo.measurements import PauliZProduct, PauliZProductInput
   from qoqo import QuantumProgram
   # initialize |psi>
   init_circuit = Circuit()
   # Apply a RotateY gate with a symbolic angle
   # To execute the circuit this symbolic parameter needs to be replaced 
   # by a real number with the help of a QuantumProgram
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
   # Read out product of Z on site 0 for register ro_x
   # (after basis change effectively a <X> measurement)
   x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])
   
   # Add a result (the expectation value of H) that is a combination of
   # the PauliProduct expectation values
   measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

   measurement = PauliZProduct(
      constant_circuit=init_circuit,
      circuits=[z_circuit, x_circuit],
      input=measurement_input,
   )

   # A quantum program is created from the measurement and "angle" is registered as
   # a free input parameter.
   # The QuantumProgram now has one free parameter that needs to set when executing it.
   # The symbolic value angle in the circuits will be replaced by that free parameter
   # during execution.
   program = QuantumProgram(
      measurement=measurement,
      input_parameter_names=["angle"],
   )
```

For details on how to execute QuantumPrograms see [Execution](execution.md).


A QuantumProgram returning unprocessed measurements
---------------------------------------------------

There also exist many use cases where end users want to receive the full measurement output without post-processing.
For example when working with external tools that expect full  measurement records or when implementing custom post-processing.
For these use cases the ``ClassicalRegister`` measurement can be used to create three dictionaries, one for all registers with bit values, one for all registers with float values and one for all registers with complex values.
Note that this measurement does not need a separate measurement input as no post-processing takes place.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   from qoqo.measurements import ClassicalRegister
   from qoqo import QuantumProgram
   # initialize |psi>
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

   measurement = ClassicalRegister(constant_circuit=init_circuit, circuits=[z_circuit, x_circuit])

   # A quantum program is created from the measurement and "angle" is registered as a free input parameter
   # The QuantumProgram now has one free parameter that needs to set when executing it.
   # The symbolic value angle in the circuits will be replaced by that free parameter during execution.
   program = QuantumProgram(measurement=measurement, input_parameter_names=["angle"])
```

For details on how to execute QuantumPrograms see [Execution](execution.md).