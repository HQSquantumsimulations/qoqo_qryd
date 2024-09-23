# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo_qryd

"""
Prototype qoqo devices for Rydberg hardware

.. autosummary::
   :toctree: generated/

   FirstDevice

"""

from typing import Optional, List, Dict, Union, Sequence
from qoqo.devices import GenericDevice

class QRydDevice:
    """Base class for Qryd devices."""

class FirstDevice(QRydDevice):
    """
    First example of a QRyd quantum device.

    At the moment, it is only a prototype showcasing the fundamental design.
    The device has a 2D grid of tweezer positions with a fixed number of rows and columns
    Each row contains a `columns` tweezer positions.
    The distance between neighbouring rows are fixed but in each row the tweezer positions can be changed.

    Args:
        number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
        number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
        qubits_per_row (List[int]): Fixed number of occupied tweezer position in each row.
                                    At the moment assumes that number of qubits in the traps is fixed. No loading/unloading once device is created.
        row_distance (float): Fixed distance between rows.
        initial_layout (np.ndarray): The starting layout (always had the index 0).
        controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
        controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
        allow_ccz_gate (Optional[bool]): Whether to allow ControlledControlledPauliZ operations in the device.
        allow_ccp_gate (Optional[bool]): Whether to allow ControlledControlledPhaseShift operations in the device.

    Raises:
        PyValueError
    """

    def __init__(
        self,
        number_rows: int,
        number_columns: int,
        qubits_per_row: List[int],
        row_distance: float,
        initial_layout,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
        allow_ccz_gate: Optional[bool],
        allow_ccp_gate: Optional[bool],
    ):
        return

    def single_qubit_gate_time(self) -> float:
        """
        Returns the gate time of a single qubit operation on this device.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available in the device.
        """

    def two_qubit_gate_time(self) -> float:
        """
        Returns the gate time of a two qubit operation on this device.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available in the device.
        """

    def three_qubit_gate_time(self) -> float:
        """
        Returns the gate time of a three qubit operation on this device.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available in the device.
        """

    def multi_qubit_gate_time(self) -> float:
        """
        Returns the gate time of a multi qubit operation on this device.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available in the device.
        """

    def phase_shift_controlled_z(self):
        """
        Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
        """

    def phase_shift_controlled_phase(self):
        """
        Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
        """

    def gate_time_controlled_z(self):
        """
        Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
        """

    def gate_time_controlled_phase(self):
        """
        Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
        """

    def generic_device(self) -> GenericDevice:
        """
        Turns Device into GenericDevice

        Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
        (for example when the interface needs to be serialized)

        Returns:
            GenericDevice: The device in generic representation

        Note:
            GenericDevice uses nested HashMaps to represent the most general device connectivity.
            The memory usage will be inefficient for devices with large qubit numbers.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the FirstDevice using the bincode crate.

        Returns:
            ByteArray: The serialized FirstDevice (in bincode form).

        Raises:
            ValueError: Cannot serialize FirstDevice to bytes.
        """

    @staticmethod
    def from_bincode(input: bytearray) -> FirstDevice:
        """
        Convert the bincode representation of the FirstDevice to a FirstDevice using the bincode crate.

        Args:
            input (ByteArray): The serialized FirstDevice (in bincode form).

        Returns:
            FirstDevice: The deserialized FirstDevice.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to FirstDevice.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the FirstDevice.

        Returns:
            str: The serialized form of FirstDevice.

        Raises:
            ValueError: Cannot serialize FirstDevice to json.
        """

    @staticmethod
    def from_json(input: str) -> FirstDevice:
        """
        Convert the json representation of a FirstDevice to a FirstDevice.

        Args:
            input (str): The serialized FirstDevice in json form.

        Returns:
            FirstDevice: The deserialized FirstDevice.

        Raises:
            ValueError: Input cannot be deserialized to FirstDevice.
        """

    def number_qubits(self) -> int:
        """
        Return number of qubits in device.

        Returns:
            int: The number of qubits.

        """

    def number_rows(self) -> int:
        """
        Return the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.

        Returns:
            int: The number of rows.

        """

    def number_columns(self) -> int:
        """
        Return number of columns in device.

        Returns:
            int: The number of columns.

        """

    def qubit_positions(self) -> Dict[int, (int, int)]:
        """
        Return the position of each qubit in the row-column grid of tweezer positions.

        Returns:
            Dict[int, (int, int)]: Map between qubit number and row-column position
        """

    def change_qubit_positions(self, new_positions: int, int):
        """
        Change the positions of the qubits in their rows.

        The occupation of the available tweezer positions can be changed.
        This allows us to change the positions of the qubits in each row.

        Args:
            new_positions (Dict[int, (int, int)]): The new column positions of the qubits, given as a map between qubits and new positions.

        Raises:
            ValueError: trying to change the number of qubits in one row
        """

    def switch_layout(self, layout_number: int):
        """
        Switch to a different pre-defined layout.

        Args:
            layout_number (int): The number index of the new layout

        Raises:
            PyValueError
        """

    def add_layout(self, layout_number: int, layout: List[float]):
        """
        Add a new layout to the device.

        A layout is a two-dimensional representation of the y-positions of the tweezers in each row.
        The x-position is fixed by the row-distance.

        Args:
            layout_number (int): The number index that is assigned to the new layout
            layout (List[float]): The new layout that is added

        Raises:
            PyValueError: layout number is already in use
        """

    def set_cutoff(self, cutoff: float):
        """
        Set distance cutoff for two-qubit gate operations.

        In the FirstQryd device the availability of two-qubit operations
        is determined by the physical distance between the involved qubits.

        When the distance is larger than the cut-off the two-qubit gate is not available.
        The cutoff defaults to 1.0 but can be changed with the set_cutoff function.

        Args:
            cutoff (float): The new cutoff for interaction distance
        """

    def _enum_to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Enum variant of the Device.

        Only used for internal interfacing.

        Returns:
            ByteArray: The serialized QrydDevice (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Device to bytes.
        """

    def two_qubit_edges(self) -> Sequence[(int, int)]:
        """
        Return the list of pairs of qubits linked by a native two-qubit-gate in the device.

        A pair of qubits is considered linked by a native two-qubit-gate if the device
        can implement a two-qubit-gate between the two qubits without decomposing it
        into a sequence of gates that involves a third qubit of the device.
        The two-qubit-gate also has to form a universal set together with the available
        single qubit gates.

        The returned vectors is a simple, graph-library independent, representation of
        the undirected connectivity graph of the device.
        It can be used to construct the connectivity graph in a graph library of the user's
        choice from a list of edges and can be used for applications like routing in quantum algorithms.

        Example
        -------

        To construct a networkx graph from this output one can use

        >>> import networkx as nx
        ... from qoqo_qryd import FirstDevice
        ...
        ... device = FirstDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
        ... edges = device.two_qubit_edges()
        ... graph = nx.Graph()
        ... graph.add_edges_from(edges)


        Returns:
            Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
        """
