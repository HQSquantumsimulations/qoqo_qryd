Rydberg devices and operations
==============================

Due to the nature of the QRydDemo hardware based on Rydberg atoms, QRydDemo quantum computing devices can have special capabilities that are 'not' present in all universal quantum computers.

The devices can have two-dimensional grids of optical tweezer-positions and different two-dimensional grids can be calibrated. A tweezer-position is a physical spot that can be populated by a qubit.
Not every tweezer position needs to be filled by a qubit and qubit can be moved between tweezer positions.
Note that all functionality described here is a preview and does not represent a finalized QRydDemo design.

Special operations
------------------

To support the full flexibility of the QRydDemo devices, two additional qoqo operations are provided ``PragmaChangeQRydLayout`` and ``PragmaShiftQRydQubit``.
``PragmaChangeQRydLayout`` allows a quantum circuit to change between predefined calibrated optical tweezer positions.
``PragmaShiftQRydQubit`` allows a quantum circuit to shift a qubit from one tweezer position to another.

.. code-block:: python

   from qoqo import Circuit
   from qoqo_qryd.pragma_operations import PragmaChangeQRydLayout, PragmaShiftQRydQubit
   circuit = Circuit()
   # Switch to predefined layout 1
   circuit += PragmaChangeQRydLayout(new_layout=1).to_pragma_change_device()
   # Shift qubit 0 to tweezer position: row 0, column 1 and qubit 1 to postion row 1, column 1
   circuit += PragmaShiftQRydQubit(new_positions={0: (0,1), 1: (1,1)}).to_pragma_change_device()


Devices
-------

Each type of QRydDemo hardware type or Simulator device can be represented by a Device class.
The available hardware operations are defined in the devices. They save the 2D connectivity and can be queried for the availability of certain gate operations on the qubit.
At the moment there is only an example Device class ``FirstDevice``, that can be used for simulations. More devices will be added as the hardware specifications are finalized.
The fundamental gates that are available on the QRydDemo devices are the ``RotateX``, ``PhaseShiftState0``, ``PhaseShiftState1`` and ``PhaseShiftedControlledZ`` qoqo operations.
The three single qubit gates are assumed to be available on all qubits. 
The ``PhaseShiftedControlledZ`` is available between a subset of qubit pairs.
The ``PhaseShiftedControlledZ`` is a ControlledPauliZ gate that also applies single qubit phases.
The phase shifts can in principle be device dependent.
The devices can optionally contain the ``controlled_z_phase`` parameter, that defines the phase shift of the two-qubit gate for the device.

For the example model of ``FirstDevice`` the two qubit gate is available between qubits that are closer than a physical cut-off distance of ``1.0``.

.. code-block:: python

   from qoqo_qryd import devices
   import numpy as np
   # create a FirstDevice
   device = devices.FirstDevice(
      # The number of tweezer position rows in the 2D Grid is fixed
      number_rows=2,
      # The number of tweezer position  columns in the 2D grid is also fixed
      number_columns=4,
      # As not all tweezer positions must be filled, the number of positions
      # occupied by qubits per row is fixed
      qubits_per_row=[2, 2],
      # The (model) physical distance between rows is fixed
      row_distance=1.0,
      # The initial layout (layout number 0 for PragmaChangeQRydLayout) is defined 
      # by the physical positions of the tweezers in each row
      initial_layout=np.array([
         [0.0, 1.0, 2.0, 3.0],
         [0.0, 1.0, 2.0, 3.0]])
      # This parameter is optional. When provided it defines the phase shift 
      # in the PhaseShiftedControlledZ gate
      controlled_z_phase=0.1)

   # Print the two-qubit-operation connectivity graph of the device
   print(device.two_qubit_edges())

SimulatorBackend
----------------

The ``SimulatorBackend`` of qoqo-qryd can execute qoqo QuantumPrograms depending on the provided devices. At the moment only the ``FirstDevice`` is available for the QRydDemo project.
Executing a circuit with the ``SimulatorBackend`` initialized by the ``FirstDevice`` corresponds to running a simulation of the QuantumProgram which validates that only
operations available in ``FirstDevice`` are used.

.. code-block:: python

   from qoqo_qryd import devices
   from qoqo_qryd import SimulatorBackend
   import numpy as np
   # create a FirstDevice
   device = devices.FirstDevice(
      # The number of tweezer position rows in the 2D Grid is fixed
      number_rows=2,
      # The number of tweezer position  columns in the 2D grid is also fixed
      number_columns=4,
      # As not all tweezer positions must be filled, the number of positions
      # occupied by qubits per row is fixed
      qubits_per_row=[2, 2],
      # The (model) physical distance between rows is fixed
      row_distance=1.0,
      # The initial layout (layout number 0 for PragmaChangeQRydLayout) is defined 
      # by the physical positions of the tweezers in each row
      initial_layout=np.array([
         [0.0, 1.0, 2.0, 3.0],
         [0.0, 1.0, 2.0, 3.0]]))

   # Initialize Backend
   backend = SimulatorBackend(device)
