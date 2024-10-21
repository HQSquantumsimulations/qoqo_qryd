# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo_qryd

"""
Tweezer devices for the QRyd platform.

.. autosummary::
   :toctree: generated/

   TweezerDevice
   TweezerMutableDevice

"""

from typing import Optional, List, Dict, Union, Sequence
from qoqo.devices import GenericDevice

class TweezerDevice:
    """
    Tweezer Device

    This interface does not allow setting any piece of information about the device
    tweezers. This class is meant to be used by the end user.

    Args:
        controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
                                      It can be hardcoded to a specific value if a float is passed in as String.
        controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
    """

    def __init__(
        self,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
        return

    @staticmethod
    def from_mutable(device: TweezerMutableDevice) -> TweezerDevice:
        """
        Creates a new TweezerDevice instance from a TweezerMutableDevice instance.

        Args:
            device (TweezerMutableDevice): The TweezerMutableDevice instance.

        Returns:
            TweezerDevice: The new TweezerDevice instance.
        """

    @staticmethod
    def from_api(
        device_name: Optional[str],
        access_token: Optional[str],
        mock_port: Optional[str],
        seed: Optional[int],
        dev: Optional[bool],
        api_version: Optional[str],
    ) -> TweezerDevice:
        """
        Creates a new TweezerDevice instance containing populated tweezer data.

        This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.

        Args:
            device_name (Optional[str]): The name of the device to instantiate. Defaults to "qryd_emulator".
            access_token (Optional[str]): An access_token is required to access QRYD hardware and emulators.
                                The access_token can either be given as an argument here
                                    or set via the environmental variable `$QRYD_API_TOKEN`.
            mock_port (Optional[str]): Server port to be used for testing purposes.
            seed (Optional[int]): Optionally overwrite seed value from downloaded device instance.
            dev (Optional[bool]): The boolean to set the dev header to.
            api_version (Optional[str]): The version of the QRYD API to use. Defaults to "v1_1".

        Returns:
            TweezerDevice: The new TweezerDevice instance with populated tweezer data.

        Raises:
            RoqoqoBackendError
        """

    def current_layout(self) -> str:
        """
        Get the name of the current layout.

        Returns:
            str: The name of the current layout.
        """

    def switch_layout(self, layout_number: str, with_trivial_map: bool):
        """
        Switch to a different pre-defined Layout.

        It is updated only if the given Layout name is present in the device's
        Layout register. If the qubit -> tweezer mapping is empty, it is
        trivially populated by default.

        Args:
            layout_number (str): The number index of the new Layout.
            with_trivial_map (bool): Whether the qubit -> tweezer mapping should be trivially populated. Defaults to true.

        Raises:
            PyValueError
        """

    def available_layouts(self) -> List[str]:
        """
        Returns a list of all available Layout names.

        Returns:
            List[str]: The list of all available Layout names.
        """

    def add_qubit_tweezer_mapping(self, qubit: int, tweezer: int) -> Dict[int, int]:
        """
        Modifies the qubit -> tweezer mapping of the device.

        If a qubit -> tweezer mapping is already present, it is overwritten.

        Args:
            qubit (int): The index of the qubit.
            tweezer (int): The index of the tweezer.

        Returns:
            dict[int, int]: The updated qubit -> tweezer mapping.

        Raises:
            ValueError: The tweezer is not present in the device.
        """

    def get_qubit_to_tweezer_mapping(self) -> Dict[int, int]:
        """
        Get the qubit -> tweezer mapping of the device.

        Returns:
            dict[int, int]: The qubit -> tweezer mapping.
            None: The mapping is empty.
        """

    def get_available_gates_names(self, layout_name: Optional[str]) -> List[str]:
        """
        Get the names of the available gates in the given layout.

        Args:
            layout_name (Optional[str]): The name of the layout. Defaults to the current Layout.

        Returns:
            list[str]: List of the names of the available gates in the given layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def get_allow_reset(self) -> bool:
        """
        Get whether the device allows PragmaActiveReset operations or not.

        Returns:
            bool: Whether the device allows PragmaActiveReset operations or not.
        """

    def deactivate_qubit(self, qubit: int) -> Dict[int, int]:
        """
        Deactivate the given qubit in the device.

        Args:
            qubit (int): The input qubit identifier.

        Returns:
            dict[int, int]: The updated qubit -> tweezer mapping.

        Raises:
            PyValueError: If the given qubit identifier is not present in the mapping.
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

    def phase_shift_controlled_z(self) -> float:
        """
        Returns the PhaseShiftedControlledZ phase shift according to the device's relation.

        Returns:
            float: The PhaseShiftedControlledZ phase shift.

        Raises:
            ValueError: Error in relation selection.
        """

    def phase_shift_controlled_phase(self) -> float:
        """
        Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.

        Returns:
            float: The PhaseShiftedControlledPhase phase shift.

        Raises:
            ValueError: Error in relation selection.
        """

    def gate_time_controlled_z(self, control: int, target: int, phi: float) -> float:
        """
        Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.

        Args:
            control (int): The control qubit the gate acts on
            target (int): The target qubit the gate acts on
            phi (float): The phi angle to be checked.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available on the device.
        """

    def gate_time_controlled_phase(
        self, control: int, target: int, phi: float, theta: float
    ) -> float:
        """
        Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.

        Args:
            control (int): The control qubit the gate acts on
            target (int): The target qubit the gate acts on
            phi (float): The phi angle to be checked.
            theta (float): The theta angle to be checked.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available on the device.
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
        Return the bincode representation of the TweezerDevice using the bincode crate.

        Returns:
            ByteArray: The serialized TweezerDevice (in bincode form).

        Raises:
            ValueError: Cannot serialize TweezerDevice to bytes.
        """

    @staticmethod
    def from_bincode(input: bytearray) -> TweezerDevice:
        """
        Convert the bincode representation of the TweezerDevice to a TweezerDevice using the bincode crate.

        Args:
            input (ByteArray): The serialized TweezerDevice (in bincode form).

        Returns:
            TweezerDevice: The deserialized TweezerDevice.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to TweezerDevice.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the TweezerDevice.

        Additionally, a gate set check is performed.

        Returns:
            str: The serialized form of TweezerDevice.

        Raises:
            ValueError: Cannot serialize TweezerDevice to json or
                the device does not have valid QRyd gates available.
        """

    @staticmethod
    def from_json(input: str) -> TweezerDevice:
        """
        Convert the json representation of a TweezerDevice to a TweezerDevice.

        If a default_layout is found in the input, a layout switch is executed.
        Additionally, a gate set check is performed.

        Args:
            input (str): The serialized TweezerDevice in json form.

        Returns:
            TweezerDevice: The deserialized TweezerDevice.

        Raises:
            ValueError: Input cannot be deserialized to TweezerDevice  or
                the device does not have valid QRyd gates available.
        """

    def number_qubits(self) -> int:
        """
        Return number of qubits in device.

        Returns:
            int: The number of qubits.
        """

    def number_tweezer_positions(self, layout_name: Optional[str]) -> int:
        """
        Returns the number of total tweezer positions in the device.

        Args:
            layout_name (Optional[str]): The name of the layout to reference. Defaults to the current layout.

        Returns:
            int: The number of tweezer positions in the device.
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

        Returns:
            Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
        """

    def two_tweezer_edges(self) -> Sequence[(int, int)]:
        """
        Returns the two tweezer edges of the device.

        And edge between two tweezer is valid only if the
        PhaseShiftedControlledPhase gate can be performed.

        Returns:
            Sequence[(int, int)]: List of two tweezer edges
        """

    def qrydbackend(self):
        """
        Returns the backend associated with the device.
        """

    def seed(self):
        """
        Returns the seed usized for the API.
        """

    def _enum_to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Enum variant of the Device.

        Only used for internal interfacing.

        Returns:
            ByteArray: The serialized TweezerDevice (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Device to bytes.
        """

    def draw(
        self,
        draw_shifts: Optional[bool],
        pixel_per_point: Optional[float],
        file_save_path: Optional[str],
    ):
        """
        Creates a graph representing a TweezerDevice.

        Args:
            draw_shifts (Optional[bool]): Whether to draw shifts or not. Default: false
            pixel_per_point (Optional[float]): The quality of the image.
            file_save_path (Optional[str]): Path to save the image to. Default: output the image with the display method.

        Raises:
            ValueError: if there is no layout, an error occurred during the compilation or and invalid path was provided.
        """

class TweezerMutableDevice:
    """
    Tweezer Mutable Device

    This interface allows setting any piece of information about the device
    tweezer.

    Args:
        controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
                                      It can be hardcoded to a specific value if a float is passed in as String.
        controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
    """

    def __init__(
        self,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
        return

    def current_layout(self) -> str:
        """
        Get the name of the current layout.

        Returns:
            str: The name of the current layout.
        """

    def add_layout(self, name: str):
        """
        Add a new layout to the device.

        Args:
            name (str): The name that is assigned to the new Layout.
        """

    def switch_layout(self, layout_number: str, with_trivial_map: bool):
        """
        Switch to a different pre-defined Layout.

        It is updated only if the given Layout name is present in the device's
        Layout register. If the qubit -> tweezer mapping is empty, it is
        trivially populated by default.

        Args:
            layout_number (str): The number index of the new Layout.
            with_trivial_map (bool): Whether the qubit -> tweezer mapping should be trivially populated. Defaults to true.

        Raises:
            PyValueError
        """

    def available_layouts(self) -> List[str]:
        """
        Returns a list of all available Layout names.

        Returns:
            List[str]: The list of all available Layout names.
        """

    def add_qubit_tweezer_mapping(self, qubit: int, tweezer: int) -> Dict[int, int]:
        """
        Modifies the qubit -> tweezer mapping of the device.

        If a qubit -> tweezer mapping is already present, it is overwritten.

        Args:
            qubit (int): The index of the qubit.
            tweezer (int): The index of the tweezer.

        Returns:
            dict[int, int]: The updated qubit -> tweezer mapping.

        Raises:
            ValueError: The tweezer is not present in the device.
        """

    def get_qubit_to_tweezer_mapping(self) -> Dict[int, int]:
        """
        Get the qubit -> tweezer mapping of the device.

        Returns:
            dict[int, int]: The qubit -> tweezer mapping.
            None: The mapping is empty.
        """

    def get_available_gates_names(self, layout_name: Optional[str]) -> List[str]:
        """
        Get the names of the available gates in the given layout.

        Args:
            layout_name (Optional[str]): The name of the layout. Defaults to the current Layout.

        Returns:
            list[str]: List of the names of the available gates in the given layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def get_allow_reset(self) -> bool:
        """
        Get whether the device allows PragmaActiveReset operations or not.

        Returns:
            bool: Whether the device allows PragmaActiveReset operations or not.
        """

    def deactivate_qubit(self, qubit: int) -> Dict[int, int]:
        """
        Deactivate the given qubit in the device.

        Args:
            qubit (int): The input qubit identifier.

        Returns:
            dict[int, int]: The updated qubit -> tweezer mapping.

        Raises:
            PyValueError: If the given qubit identifier is not present in the mapping.
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

    def phase_shift_controlled_z(self) -> float:
        """
        Returns the PhaseShiftedControlledZ phase shift according to the device's relation.

        Returns:
            float: The PhaseShiftedControlledZ phase shift.

        Raises:
            ValueError: Error in relation selection.
        """

    def phase_shift_controlled_phase(self) -> float:
        """
        Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.

        Returns:
            float: The PhaseShiftedControlledPhase phase shift.

        Raises:
            ValueError: Error in relation selection.
        """

    def gate_time_controlled_z(self, control: int, target: int, phi: float) -> float:
        """
        Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.

        Args:
            control (int): The control qubit the gate acts on
            target (int): The target qubit the gate acts on
            phi (float): The phi angle to be checked.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available on the device.
        """

    def gate_time_controlled_phase(
        self, control: int, target: int, phi: float, theta: float
    ) -> float:
        """
        Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.

        Args:
            control (int): The control qubit the gate acts on
            target (int): The target qubit the gate acts on
            phi (float): The phi angle to be checked.
            theta (float): The theta angle to be checked.

        Returns:
            float: The gate time.

        Raises:
            ValueError: The gate is not available on the device.
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
        Return the bincode representation of the TweezerMutableDevice using the bincode crate.

        Returns:
            ByteArray: The serialized TweezerMutableDevice (in bincode form).

        Raises:
            ValueError: Cannot serialize TweezerMutableDevice to bytes.
        """

    @staticmethod
    def from_bincode(input: bytearray) -> TweezerMutableDevice:
        """
        Convert the bincode representation of the TweezerMutableDevice to an
        TweezerMutableDevice using the bincode crate.

        Args:
            input (ByteArray): The serialized TweezerMutableDevice (in bincode form).

        Returns:
            TweezerMutableDevice: The deserialized TweezerMutableDevice.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to TweezerMutableDevice.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the TweezerMutableDevice.

        Additionally, a gate set check is performed.

        Returns:
            str: The serialized form of TweezerMutableDevice.

        Raises:
            ValueError: Cannot serialize TweezerMutableDevice to json or
                the device does not have valid QRyd gates available.
        """

    @staticmethod
    def from_json(input: str) -> TweezerMutableDevice:
        """
        Convert the json representation of a TweezerMutableDevice to an TweezerMutableDevice.

        Additionally, a gate set check is performed.

        Args:
            input (str): The serialized TweezerMutableDevice in json form.

        Returns:
            TweezerMutableDevice: The deserialized TweezerMutableDevice.

        Raises:
            ValueError: Input cannot be deserialized to TweezerMutableDevice or
                the device does not have valid QRyd gates available.
        """

    def number_qubits(self) -> int:
        """
        Return number of qubits in device.

        Returns:
            int: The number of qubits.

        """

    def number_tweezer_positions(self, layout_name: Optional[str]) -> int:
        """
        Returns the number of total tweezer positions in the device.

        Args:
            layout_name (Optional[str]): The name of the layout to reference. Defaults to the current layout.

        Returns:
            int: The number of tweezer positions in the device.
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

        Returns:
            Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
        """

    def two_tweezer_edges(self) -> Sequence[(int, int)]:
        """
        Returns the two tweezer edges of the device.

        And edge between two tweezer is valid only if the
        PhaseShiftedControlledPhase gate can be performed.

        Returns:
            Sequence[(int, int)]: List of two tweezer edges
        """

    def qrydbackend(self):
        """
        Returns the backend associated with the device.
        """

    def seed(self):
        """
        Returns the seed usized for the API.
        """

    def _enum_to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Enum variant of the Device.

        Only used for internal interfacing.

        Returns:
            ByteArray: The serialized TweezerMutableDevice (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Device to bytes.
        """

    def set_tweezer_single_qubit_gate_time(
        self, hqslang: str, tweezer: int, gate_time: float, layout_name: Optional[str]
    ):
        """
        Set the time of a single-qubit gate for a tweezer in a given Layout.

        Args:
            hqslang (str): The hqslang name of a single-qubit gate.
            tweezer (int): The index of the tweezer.
            gate_time (float): The the gate time for the given gate.
            layout_name (Optional[str]): The name of the Layout to apply the gate time in.
                Defaults to the current Layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def set_tweezer_two_qubit_gate_time(
        self,
        hqslang: str,
        tweezer0: int,
        tweezer1: int,
        gate_time: float,
        layout_name: Optional[str],
    ):
        """
        Set the time of a two-qubit gate for a tweezer couple in a given Layout.

        Args:
            hqslang (str): The hqslang name of a single-qubit gate.
            tweezer0 (int): The index of the first tweezer.
            tweezer1 (int): The index of the second tweezer.
            gate_time (float): The the gate time for the given gate.
            layout_name (Optional[str]): The name of the Layout to apply the gate time in.
                Defaults to the current Layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def set_tweezer_three_qubit_gate_time(
        self,
        hqslang: str,
        tweezer0: int,
        tweezer1: int,
        tweezer2: int,
        gate_time: float,
        layout_name: Optional[str],
    ):
        """
        Set the time of a three-qubit gate for a tweezer trio in a given Layout.

        Args:
            hqslang (str): The hqslang name of a three-qubit gate.
            tweezer0 (int): The index of the first tweezer.
            tweezer1 (int): The index of the second tweezer.
            tweezer2 (int): The index of the third tweezer.
            gate_time (float): The the gate time for the given gate.
            layout_name (Optional[str]): The name of the Layout to apply the gate time in.
                Defaults to the current Layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def set_tweezer_multi_qubit_gate_time(
        self,
        hqslang: str,
        tweezers: List[int],
        gate_time: float,
        layout_name: Optional[str],
    ):
        """
        Set the time of a multi-qubit gate for a list of tweezers in a given Layout.

        Args:
            hqslang (str): The hqslang name of a multi-qubit gate.
            tweezers (List[int]): The list of tweezer indexes.
            gate_time (float): The the gate time for the given gate.
            layout_name (Optional[str]): The name of the Layout to apply the gate time in.
                Defaults to the current Layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def set_allowed_tweezer_shifts(
        self, tweezer: int, allowed_shifts: list[list[int]], layout_name: Optional[str]
    ):
        """
        Set the allowed Tweezer shifts of a specified Tweezer.

        The tweezer give the tweezer a qubit can be shifted out of. The values are lists
        over the directions the qubit in the tweezer can be shifted into.
        The items in the list give the allowed tweezers the qubit can be shifted into in order.
        For a list 1,2,3 the qubit can be shifted into tweezer 1, into tweezer 2 if tweezer 1 is not occupied,
        and into tweezer 3 if tweezer 1 and 2 are not occupied.

        Args:
            tweezer (int): The index of the tweezer.
            allowed_shifts (list[list[int]]): The allowed tweezer shifts.
            layout_name (Optional[str]): The name of the Layout to apply the allowed shifts in.
                Defaults to the current Layout.

        Raises:
            ValueError: The tweezer or shifts are not present in the device or
                the given tweezer is contained in the shift list.
        """

    def set_allowed_tweezer_shifts_from_rows(
        self, row_shifts: list[list[int]], layout_name: Optional[str]
    ):
        """
        Set the allowed Tweezer shifts from a list of tweezers.

        The items in the rows give the allowed tweezers that qubit can be shifted into.
        For a row defined as 1,2,3, a qubit in tweezer 1 can be shifted into tweezer 2,
        and into tweezer 3 if tweezer 2 is not occupied by a qubit.

        Args:
            row_shifts (list[list[int]]): A list of lists, each representing a row of tweezers.
            layout_name (Optional[str]): The name of the Layout to apply the allowed shifts in.
                Defaults to the current Layout.

        Raises:
            ValueError: The involved tweezers are not present in the device.
        """

    def set_tweezers_per_row(tweezers_per_row: List[int], layout_name: Optional[str], self):
        """
        Set the tweezer per row value for a given Layout.

        This is needed for dynamically switching layouts during circuit execution.
        Only switching between layouts having the same tweezer per row value is supported.

        Args:
            tweezers_per_row(List[int]): Vector containing the number of tweezers per row to set.
            layout_name(Optional[str]): The name of the Layout to set the tweezer per row for. Defaults to the current Layout.

        Raises:
            ValueError: No layout name provided and no current layout set.
        """

    def set_allow_reset(self, allow_reset: bool):
        """
        Set whether the device allows PragmaActiveReset operations or not.

        Args:
            allow_reset (bool): Whether the device should allow PragmaActiveReset operations or not.

        Raises:
            ValueError: The device isn't compatible with PragmaActiveReset.
        """

    def set_default_layout(self, layout: str):
        """
        Set the name of the default layout to use and switch to it.

        Args:
            layout (str): The name of the layout to use.

        Raises:
            ValueError: The given layout name is not present in the layout register.
        """

    def draw(
        self,
        draw_shifts: Optional[bool],
        pixel_per_point: Optional[float],
        file_save_path: Optional[str],
    ):
        """
        Creates a graph representing a TweezerDevice.

        Args:
            draw_shifts (Optional[bool]): Whether to draw shifts or not. Default: false
            pixel_per_point (Optional[float]): The quality of the image.
            file_save_path (Optional[str]): Path to save the image to. Default: output the image with the display method.

        Raises:
            ValueError: if there is no layout, an error occurred during the compilation or and invalid path was provided.
        """
