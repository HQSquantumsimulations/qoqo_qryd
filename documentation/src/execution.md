Executing Quantum Programs
==========================

To obtain results from a QuantumProgram, Measurement or Circuit it needs to be executed on real quantum computing hardware or run on a simulator.

Qoqo uses separate backends for this evaluation. For each hardware or simulator a backend can be created that implements qoqo's ``EvaluatingBackend`` interface and runs QuantumPrograms. For an overview of backends see the [qoqo](https://github.com/HQSquantumsimulations/qoqo) website. Backends which provide the functionality to run a single circuit are so-called ``EvaluatingBackend``. The QRydDemo backends fall in this category.

An ``EvaluatingBackend`` can run:

1. **A single circuit**. The backend will execute just the circuit and return the measurement results of all registers in a tuple (bit-registers, float-registers, complex-registers). bit_registers is a dictionary of all registers with bit values, float_registers of all registers with float values and complex_registers of all registers with complex values. All the post-processing needs to be done manually.

2. **A measurement**. All circuits in the measurement are run and the post-processed expectation values are returned.

3. **A quantum program**. A ``QuantumProgram`` also handles replacement of variables. It provides its own ``run`` method and calls a provided backend internally.

As an example we will use the quantum program from [Introduction](introduction.md) and the [qoqo-quest](https://github.com/HQSquantumsimulations/qoqo-quest) simulator backend. Here we show three alternative options that can be ran: a single circuit, a measurement, and a quantum program.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   from qoqo.measurements import PauliZProduct, PauliZProductInput
   from qoqo import QuantumProgram
   from qoqo_quest import Backend
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

   # Preparing the measurement input for one qubit
   measurement_input = PauliZProductInput(1, False)
   # Read out product of Z on site 0 for register ro_z (no basis change)
   z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
   # Read out product of Z on site 0 for register ro_x
   # (after basis change effectively a <X> measurement)
   x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])
   
   # Add a result (the expectation value of H) that is a combination of the PauliProduct
   # expectation values
   measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

   measurement = PauliZProduct(
      constant_circuit=init_circuit,
      circuits=[z_circuit, x_circuit],
      input=measurement_input,
   )

   # Here we show three alternative options that can be ran:
   # a single circuit, a measurement, and a quantum program.

   # Create a backend simulating one qubit
   backend = Backend(1)

   # a) Run a single circuit 
   (bit_registers, float_registers, complex_registers) = backend.run_circuit(z_circuit)

   # b) To run a measurement we need to replace the free parameter by hand
   executable_measurement = measurement.substitute_parameters({"angle": 0.2})
   expecation_values = backend.run_measurement(executable_measurement)
   print(expecation_values)

   # c) Run a quantum program
   # The QuantumProgram now has one free parameter that needs to bet set when executing it.
   # The symbolic value "angle" in the circuits will be replaced by that free parameter
   # during execution.
   program = QuantumProgram(measurement=measurement, input_parameter_names=["angle"])
   # Run the program with  0.1 substituting `angle`
   expecation_values = program.run(backend, [0.1])
```

Note: The QuantumProgram can be run in the same way with the qoqo_qryd ``SimulatorBackend`` when all quantum operations are replaced by sequences of operations directly supported by the QRydDemo hardware. However, in order to use the qoqo_qryd ``SimulatorBackend``, a device needs to be defined first, as shown in the SimulatorBackend subsection of [QRyd Specifics](qrydspecifics.md).


In general, to distinguish between a command returning expectation values and a program returning register the command ``run_registers`` is used here.

```python
   from qoqo import Circuit
   from qoqo import operations as ops
   from qoqo.measurements import ClassicalRegister
   from qoqo import QuantumProgram
   from qoqo_quest import Backend
   # initialize |psi>
   init_circuit = Circuit()
   # Apply a RotateY gate with a symbolic angle
   # To execute the circuit this symbolic parameter needs to be replaced 
   # with a real number with the help of a QuantumProgram
   init_circuit += ops.RotateY(0, "angle")
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

   # A quantum program is created from the measurement and "angle" is registered as a free input
   # parameter. The QuantumProgram now has one free parameter that needs to be set when
   # executing it. The symbolic value angle in the circuits will be replaced by that free parameter
   # during execution.
   program = QuantumProgram(measurement=measurement, input_parameter_names=["angle"])

   backend = Backend(1)
   (bit_registers, float_registers, complex_registers) = program.run_registers(backend, [0.1])
   print(bit_registers)
```


Executing QuantumPrograms without returning expecation values
---------------------------------------------------------------------

As described in [Introduction](introduction.md) the ``ClassicalRegister`` measurement can be used to return the full measurement record. 

Non-executing backends
----------------------

Qoqo also has backends that cannot be used to run or evaluate a quantum circuit. These backends typically are used to translate qoqo circuits to other quantum toolkits or languages. One example is [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm)