# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo_qryd

"""
QRyd specific PragmaOperations that support changing the QRyd device during a circuit evaluation

.. autosummary::
   :toctree: generated/

   PragmaChangeQRydLayout
   PragmaShiftQRydQubit
   PragmaDeactivateQRydQubit
   PragmaShiftQubitsTweezers
   PragmaSwitchDeviceLayout
"""

from typing import List, Tuple, Dict, Set

class PragmaChangeQRydLayout:
    """
    This PRAGMA operation changes the layout of a QRyd device.

    Before running a circuit a number of layouts can be registered
    in the device with the `add_layout` method.

    This PRAGMA operation switches between the predefined operations.

    Args:
        new_layout (int): The index of the new layout.
    """

    def __init__(self, new_layout: int):
        return

    def new_layout(self) -> int:
        """
        Return the index of the new layout the Pragma changes the device to.

        Returns:
            int: The index of the layout.
        """

    def to_pragma_change_device(self):
        """
        Wrap PragmaChangeQRydLayout in PragmaChangeDevice operation

        PragmaChangeQRydLayout is device specific and can not be directly added to a Circuit.
        Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
        to the circuit.

        Example
        -------

        >>> from qoqo import Circuit
        ... from qoqo_qryd.pragma_operations import PragmaChangeQRydLayout
        ... circuit = Circuit()
        ... circuit += PragmaChangeQRydLayout(new_layout=1).to_pragma_change_device()

        Returns:
            PragmaChangeDevice
        """

    def involved_qubits(self) -> Set[int]:
        """
        List all involved qubits (here, all).

        Returns:
            set[int]: The involved qubits of the PRAGMA operation.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PragmaChangeQRydLayout using the bincode crate.

        Returns:
            ByteArray: The serialized Circuit (in bincode form).

        Raises:
            ValueError: Cannot serialize PragmaChangeQRydLayout to bytes.
        """

    def from_bincode(self, input: bytearray) -> PragmaChangeQRydLayout:
        """
        Convert the bincode representation of the PragmaChangeQRydLayout to a PragmaChangeQRydLayout using the bincode crate.

        Args:
            input (ByteArray): The serialized PragmaChangeQRydLayout (in bincode form).

        Returns:
            PragmaChangeQRydLayout: The deserialized PragmaChangeQRydLayout.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PragmaChangeQRydLayout.
        """

    def tags(self) -> List[str]:
        """
        Return tags classifying the type of the operation.

        Used for the type based dispatch in ffi interfaces.

        Returns:
            list[str]: The tags of the operation.
        """

    def hqslang(self) -> str:
        """
        Return hqslang name of the operation.

        Returns:
            str: The hqslang name of the operation.
        """

    def is_parametrized(self) -> bool:
        """
        Return true when the operation has symbolic parameters.

        Returns:
            bool: True if the operation contains symbolic parameters, False if it does not.
        """

    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> PragmaChangeQRydLayout:
        """
        Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

        Args:
            substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation operation with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaChangeQRydLayout:
        """
        Remap qubits in a clone of the PRAGMA operation.

        Args:
            mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed.
        """

class PragmaShiftQRydQubit:
    """
    This PRAGMA operation shifts qubits between tweezer positions.

    The tweezer positions in a FirstQryd device do not all have to be occupied.
    In a partially occupied device the qubits can be shifted between positions inside a row.
    The shift is defined by giving a mapping of qubit number and new row-column positions.

    Args:
        new_positions (Dict[int, (int, int)]): The new positions of the qubits.
    """

    def __init__(self, new_positions: int, int):
        return

    def new_positions(self) -> Dict[int, (int, int)]:
        """
        Return the map of qubit numbers to new positions in the QRyd device.

        The new positions are the

        Returns:
            Dict[int, (int, int)]: Map of qubits to new positions in the 2d grid.
        """

    def to_pragma_change_device(self):
        """
        Wrap PragmaShiftQRydQubit in PragmaChangeDevice operation

        PragmaShiftQRydQubit is device specific and can not be directly added to a Circuit.
        Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
        to the circuit.

        Example
        -------

        >>> from qoqo import Circuit
        ... from qoqo_qryd.pragma_operations import PragmaShiftQRydQubit
        ... circuit = Circuit()
        ... circuit += PragmaShiftQRydQubit(new_layout=1).to_pragma_change_device()

        Returns:
            PragmaChangeDevice
        """

    def involved_qubits(self) -> Set[int]:
        """
        List all involved qubits (here, all).

        Returns:
            set[int]: The involved qubits of the PRAGMA operation.
        """

    def tags(self) -> List[str]:
        """
        Return tags classifying the type of the operation.

        Used for the type based dispatch in ffi interfaces.

        Returns:
            list[str]: The tags of the operation.
        """

    def hqslang(self) -> str:
        """
        Return hqslang name of the operation.

        Returns:
            str: The hqslang name of the operation.
        """

    def is_parametrized(self) -> bool:
        """
        Return true when the operation has symbolic parameters.

        Returns:
            bool: True if the operation contains symbolic parameters, False if it does not.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PragmaShiftQRydQubit using the bincode crate.

        Returns:
            ByteArray: The serialized PragmaShiftQRydQubit (in bincode form).

        Raises:
            ValueError: Cannot serialize PragmaShiftQRydQubit to bytes.
        """

    def from_bincode(self, input: bytearray) -> PragmaShiftQRydQubit:
        """
        Convert the bincode representation of the PragmaShiftQRydQubit to a PragmaShiftQRydQubit using the bincode crate.

        Args:
            input (ByteArray): The serialized PragmaShiftQRydQubit (in bincode form).

        Returns:
            PragmaShiftQRydQubit: The deserialized PragmaShiftQRydQubit.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PragmaShiftQRydQubit.
        """

    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> PragmaShiftQRydQubit:
        """
        Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

        Args:
            substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation operation with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaShiftQRydQubit:
        """
        Remap qubits in a clone of the PRAGMA operation.

        Args:
            mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed.
        """

class PragmaDeactivateQRydQubit:
    """
    This PRAGMA Operation deactivates a qubit in a QRyd Experimental device.

    In QRyd Experimental devices a quantum state is trapped within an optical tweezer.
    This Operation signals the device to drop the quantum state related to the given qubit.

    Args:
        qubit (int): The qubit to deactivate.
    """

    def __init__(self, qubit: int):
        return

    def qubit(self) -> int:
        """
        Return the qubit involved in the Operation.

        Returns:
            int: The qubit involved in the Operation.
        """

    def to_pragma_change_device(self):
        """
        Wrap PragmaDeactivateQRydQubit in PragmaChangeDevice operation

        PragmaDeactivateQRydQubit is device specific and can not be directly added to a Circuit.
        Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
        to the circuit.

        Example
        -------

        >>> from qoqo import Circuit
        ... from qoqo_qryd.pragma_operations import PragmaDeactivateQRydQubit
        ... circuit = Circuit()
        ... circuit += PragmaDeactivateQRydQubit(qubit=0).to_pragma_change_device()

        Returns:
            PragmaChangeDevice
        """

    def involved_qubits(self) -> Set[int]:
        """
        List all involved qubits (here, all).

        Returns:
            set[int]: The involved qubits of the PRAGMA operation.
        """

    def tags(self) -> List[str]:
        """
        Return tags classifying the type of the operation.

        Used for the type based dispatch in ffi interfaces.

        Returns:
            list[str]: The tags of the operation.
        """

    def hqslang(self) -> str:
        """
        Return hqslang name of the operation.

        Returns:
            str: The hqslang name of the operation.
        """

    def is_parametrized(self) -> bool:
        """
        Return true when the operation has symbolic parameters.

        Returns:
            bool: True if the operation contains symbolic parameters, False if it does not.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PragmaDeactivateQRydQubit using the bincode crate.

        Returns:
            ByteArray: The serialized PragmaDeactivateQRydQubit (in bincode form).

        Raises:
            ValueError: Cannot serialize PragmaDeactivateQRydQubit to bytes.
        """

    def from_bincode(self, input: bytearray) -> PragmaDeactivateQRydQubit:
        """
        Convert the bincode representation of the PragmaDeactivateQRydQubit to a PragmaDeactivateQRydQubit using the bincode crate.

        Args:
            input (ByteArray): The serialized PragmaDeactivateQRydQubit (in bincode form).

        Returns:
            PragmaDeactivateQRydQubit: The deserialized PragmaDeactivateQRydQubit.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PragmaDeactivateQRydQubit.
        """

    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> PragmaDeactivateQRydQubit:
        """
        Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

        Args:
            substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation operation with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaDeactivateQRydQubit:
        """
        Remap qubits in a clone of the PRAGMA operation.

        Args:
            mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed.
        """

class PragmaShiftQubitsTweezers:
    """
    This PRAGMA Operation lists the shift operations to be executed in a QRyd Tweezer device.

    Each tuple contains first the starting tweezer identifier and second the ending tweezer identifier.
    Multiple instances indicate parallel operations.

    Args:
        shifts (list((int, int))): The list of shifts that can run in parallel.
    """

    def __init__(self, shifts: int, int):
        return

    def shifts(self) -> List[Tuple[int, int]]:
        """
        Return the shifts involved in the Operation.

        Returns:
            list[Tuple[int, int]]: The shifts involved in the Operation.
        """

    def to_pragma_change_device(self):
        """
        Wrap PragmaShiftQubitsTweezers in PragmaChangeDevice operation

        PragmaShiftQubitsTweezers is device specific and can not be directly added to a Circuit.
        Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
        to the circuit.

        Example
        -------

        >>> from qoqo import Circuit
        ... from qoqo_qryd.pragma_operations import PragmaShiftQubitsTweezers
        ... circuit = Circuit()
        ... circuit += PragmaShiftQubitsTweezers(shifts=[(0, 1)]).to_pragma_change_device()

        Returns:
            PragmaChangeDevice
        """

    def involved_qubits(self) -> Set[int]:
        """
        List all involved qubits (here, all).

        Returns:
            set[int]: The involved qubits of the PRAGMA operation.
        """

    def tags(self) -> List[str]:
        """
        Return tags classifying the type of the operation.

        Used for the type based dispatch in ffi interfaces.

        Returns:
            list[str]: The tags of the operation.
        """

    def hqslang(self) -> str:
        """
        Return hqslang name of the operation.

        Returns:
            str: The hqslang name of the operation.
        """

    def is_parametrized(self) -> bool:
        """
        Return true when the operation has symbolic parameters.

        Returns:
            bool: True if the operation contains symbolic parameters, False if it does not.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PragmaShiftQubitsTweezers using the bincode crate.

        Returns:
            ByteArray: The serialized PragmaShiftQubitsTweezers (in bincode form).

        Raises:
            ValueError: Cannot serialize PragmaShiftQubitsTweezers to bytes.
        """

    def from_bincode(self, input: bytearray) -> PragmaShiftQubitsTweezers:
        """
        Convert the bincode representation of the PragmaShiftQubitsTweezers to a PragmaShiftQubitsTweezers using the bincode crate.

        Args:
            input (ByteArray): The serialized PragmaShiftQubitsTweezers (in bincode form).

        Returns:
            PragmaShiftQubitsTweezers: The deserialized PragmaShiftQubitsTweezers.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PragmaShiftQubitsTweezers.
        """

    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> PragmaShiftQubitsTweezers:
        """
        Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

        Args:
            substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation operation with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaShiftQubitsTweezers:
        """
        Remap qubits in a clone of the PRAGMA operation.

        Args:
            mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed.
        """

class PragmaSwitchDeviceLayout:
    """
    This PRAGMA operation changes the layout of a Tweezer device.

    Before running a circuit a number of layouts can be registered
    in the device with the `add_layout` method.

    This PRAGMA operation switches between the predefined operations.

    Args:
        new_layout (str): The name of the new layout.
    """

    def __init__(self, new_layout: str):
        return

    def new_layout(self) -> int:
        """
        Return the name of the new layout the Pragma changes the device to.

        Returns:
            int: The name of the layout.
        """

    def to_pragma_change_device(self):
        """
        Wrap PragmaSwitchDeviceLayout in PragmaChangeDevice operation

        PragmaSwitchDeviceLayout is device specific and can not be directly added to a Circuit.
        Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
        to the circuit.

        Example
        -------

        >>> from qoqo import Circuit
        ... from qoqo_qryd.pragma_operations import PragmaSwitchDeviceLayout
        ... circuit = Circuit()
        ... circuit += PragmaSwitchDeviceLayout(new_layout="Square").to_pragma_change_device()

        Returns:
            PragmaChangeDevice
        """

    def involved_qubits(self) -> Set[int]:
        """
        List all involved qubits (here, all).

        Returns:
            set[int]: The involved qubits of the PRAGMA operation.
        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PragmaSwitchDeviceLayout using the bincode crate.

        Returns:
            ByteArray: The serialized Circuit (in bincode form).

        Raises:
            ValueError: Cannot serialize PragmaSwitchDeviceLayout to bytes.
        """

    def from_bincode(self, input: bytearray) -> PragmaSwitchDeviceLayout:
        """
        Convert the bincode representation of the PragmaSwitchDeviceLayout to
        a PragmaSwitchDeviceLayout using the bincode crate.

        Args:
            input (ByteArray): The serialized PragmaSwitchDeviceLayout (in bincode form).

        Returns:
            PragmaSwitchDeviceLayout: The deserialized PragmaSwitchDeviceLayout.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PragmaSwitchDeviceLayout.
        """

    def tags(self) -> List[str]:
        """
        Return tags classifying the type of the operation.

        Used for the type based dispatch in ffi interfaces.

        Returns:
            list[str]: The tags of the operation.
        """

    def hqslang(self) -> str:
        """
        Return hqslang name of the operation.

        Returns:
            str: The hqslang name of the operation.
        """

    def is_parametrized(self) -> bool:
        """
        Return true when the operation has symbolic parameters.

        Returns:
            bool: True if the operation contains symbolic parameters, False if it does not.
        """

    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> PragmaSwitchDeviceLayout:
        """
        Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

        Args:
            substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation operation with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaSwitchDeviceLayout:
        """
        Remap qubits in a clone of the PRAGMA operation.

        Args:
            mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

        Returns:
            self: The PRAGMA operation with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed.
        """
