# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo_qryd

"""
Devices available on the QRydDemo WebAPI.

.. autosummary::
   :toctree: generated/

   QrydEmuSquareDevice
   QrydEmuTriangularDevice

"""

from typing import Optional, Union, Sequence
from qoqo.devices import GenericDevice
from .qryd_devices import QRydDevice  # type: ignore

class QrydEmuSquareDevice(QRydDevice):
    """
    QRyd quantum device having a squared configuration.

    Provides an emulated quantum computing device with up to 30 qubits
    that can be accessed via the QRyd WebAPI.

    Args:
        seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
        controlled_z_phase_relation (Optional[Union[str, float]]): The String used to choose what kind of phi-theta relation
                                                    to use for the PhaseShiftedControlledZ gate
        controlled_phase_phase_relation (Optional[Union[str, float]]): The String used to choose what kind of phi-theta relation
                                                        to use for the PhaseShiftedControlledPhase gate
    """

    def __init__(
        self,
        seed: int,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
        return

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
        Return the bincode representation of the QrydEmuSquareDevice using the bincode crate.

        Returns:
            ByteArray: The serialized QrydEmuSquareDevice (in bincode form).

        Raises:
            ValueError: Cannot serialize QrydEmuSquareDevice to bytes.
        """

    @staticmethod
    def from_bincode(input: bytearray) -> QrydEmuSquareDevice:
        """
        Convert the bincode representation of the QrydEmuSquareDevice to a QrydEmuSquareDevice using the bincode crate.

        Args:
            input (ByteArray): The serialized QrydEmuSquareDevice (in bincode form).

        Returns:
            QrydEmuSquareDevice: The deserialized QrydEmuSquareDevice.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to QrydEmuSquareDevice.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the QrydEmuSquareDevice.

        Returns:
            str: The serialized form of QrydEmuSquareDevice.

        Raises:
            ValueError: Cannot serialize QrydEmuSquareDevice to json.
        """

    @staticmethod
    def from_json(input: str) -> QrydEmuSquareDevice:
        """
        Convert the json representation of a QrydEmuSquareDevice to a QrydEmuSquareDevice.

        Args:
            input (str): The serialized QrydEmuSquareDevice in json form.

        Returns:
            QrydEmuSquareDevice: The deserialized QrydEmuSquareDevice.

        Raises:
            ValueError: Input cannot be deserialized to QrydEmuSquareDevice.
        """

    def number_qubits(self) -> int:
        """
        Return number of qubits in device.

        Returns:
            int: The number of qubits.
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
        ... from qoqo_qryd import QrydEmuSquareDevice
        ...
        ... device = QrydEmuSquareDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
        ... edges = device.two_qubit_edges()
        ... graph = nx.Graph()
        ... graph.add_edges_from(edges)


        Returns:
            Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
        """

    def qrydbackend(self):
        """
        Returns the backend associated with the device.
        """

    def seed(self):
        """
        Returns the seed usized for the API.
        """

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

class QrydEmuTriangularDevice:
    """
    QRyd quantum device having a triangular configuration.

    Provides an emulated quantum computing device with up to 30 qubits
    that can be accessed via the QRyd WebAPI.

    Args:
        seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
        controlled_z_phase_relation (Optional[Union[str, float]]): The String used to choose what kind of phi-theta relation
                                                    to use for the PhaseShiftedControlledZ gate.
        controlled_phase_phase_relation (Optional[Union[str, float]]): The String used to choose what kind of phi-theta relation
                                                        to use for the PhaseShiftedControlledPhase gate.
        allow_ccz_gate (Optional[bool]): Whether to allow ControlledControlledPauliZ operations in the device.
        allow_ccp_gate (Optional[bool]): Whether to allow ControlledControlledPhaseShift operations in the device.
    """

    def __init__(
        self,
        seed: int,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
        allow_ccz_gate: Optional[bool],
        allow_ccp_gate: Optional[bool],
    ):
        return

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the QrydEmuTriangularDevice using the bincode crate.

        Returns:
            ByteArray: The serialized QrydEmuTriangularDevice (in bincode form).

        Raises:
            ValueError: Cannot serialize QrydEmuTriangularDevice to bytes.
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

    @staticmethod
    def from_bincode(input: bytearray) -> QrydEmuTriangularDevice:
        """
        Convert the bincode representation of the QrydEmuTriangularDevice to a QrydEmuTriangularDevice the bincode crate.

        Args:
            input (ByteArray): The serialized QrydEmuTriangularDevice (in bincode form).

        Returns:
            QrydEmuTriangularDevice: The deserialized QrydEmuTriangularDevice.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to QrydEmuTriangularDevice.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the QrydEmuTriangularDevice.

        Returns:
            str: The serialized form of QrydEmuTriangularDevice.

        Raises:
            ValueError: Cannot serialize QrydEmuTriangularDevice to json.
        """

    @staticmethod
    def from_json(input: str) -> QrydEmuTriangularDevice:
        """
        Convert the json representation of a QrydEmuTriangularDevice to a QrydEmuTriangularDevice.

        Args:
            input (str): The serialized QrydEmuTriangularDevice in json form.

        Returns:
            QrydEmuTriangularDevice: The deserialized QrydEmuTriangularDevice.

        Raises:
            ValueError: Input cannot be deserialized to QrydEmuTriangularDevice.
        """

    def number_qubits(self) -> int:
        """
        Return number of qubits in device.

        Returns:
            int: The number of qubits.

        """

    def _enum_to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Enum variant of the Device.

        Only used for internal interfacing.

        Returns:
            ByteArray: The serialized device (in [bincode] form).

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
        ... from qoqo_qryd import QrydEmuTriangularDevice
        ...
        ... device = QrydEmuTriangularDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
        ... edges = device.two_qubit_edges()
        ... graph = nx.Graph()
        ... graph.add_edges_from(edges)


        Returns:
            Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
        """

    def qrydbackend(self):
        """
        Returns the backend associated with the device.
        """

    def seed(self):
        """
        Returns the seed usized for the API.
        """

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
