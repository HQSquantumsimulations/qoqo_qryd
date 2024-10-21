# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo_qryd

"""
QRyd utilities for qoqo quantum computation toolkit.

qoqo is the HQS python package to represent quantum circuits.

.. autosummary::
    :toctree: generated/

    api_devices
    Backend
    pragma_operations
    qryd_devices
    tweezer_devices


"""

from typing import Optional, List, Tuple, Dict, Union
from qoqo import Circuit
from qoqo.measurements import (
    ClassicalRegister,
    Cheated,
    CheatedPauliZProduct,
    PauliZProduct,
)
from .tweezer_devices import TweezerDevice  # type: ignore
from .qryd_devices import QRydDevice  # type: ignore

class SimulatorBackend:
    """
    Local simulator backend for Rydberg devices.

    The QRyd simulator backend applies each operation in a circuit to a quantum register.
    The underlying simulator uses the QuEST library.
    Although the underlying simulator supports arbitrary unitary gates, the QRyd simulator only
    allows operations that are available on a device model of a QRyd device.
    This limitation is introduced by design to check the compatibility of circuits with a model of the QRyd hardware.
    For unrestricted simulations use the backend simulator of the roqoqo-quest crate.


    The simulator backend implements the qoqo EvaluatingBackend interface
    and is compatible with running single circuits, running and evaluating measurements
    and running QuantumPrograms on simulated QRyd devices.

    Args:
        device (Union[QRydDevice,TweezerDevice]): The device providing information about the available operations.

    Raises:
        TypeError: Device Parameter is not QRydDevice or TweezerDevice
    """

    def __init__(self, device: Union[QRydDevice, TweezerDevice]):
        return

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the SimulatorBackend using the bincode crate.

        Returns:
            ByteArray: The serialized SimulatorBackend (in bincode form).

        Raises:
            ValueError: Cannot serialize SimulatorBackend to bytes.
        """

    def from_bincode(self, input: bytearray) -> SimulatorBackend:
        """
        Convert the bincode representation of the SimulatorBackend to a SimulatorBackend using the bincode crate.

        Args:
            input (ByteArray): The serialized SimulatorBackend (in bincode form).

        Returns:
            SimulatorBackend: The deserialized SimulatorBackend.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to SimulatorBackend.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the SimulatorBackend.

        Returns:
            str: The serialized form of SimulatorBackend.

        Raises:
            ValueError: Cannot serialize SimulatorBackend to json.
        """

    def from_json(self, input: str) -> SimulatorBackend:
        """
        Convert the json representation of a SimulatorBackend to a SimulatorBackend.

        Args:
            input (str): The serialized SimulatorBackend in json form.

        Returns:
            SimulatorBackend: The deserialized SimulatorBackend.

        Raises:
            ValueError: Input cannot be deserialized to SimulatorBackend.
        """

    def run_circuit(self, circuit: Circuit) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """
        Run a circuit with the QRyd backend.

        A circuit is passed to the backend and executed.
        During execution values are written to and read from classical registers
        (List[bool], List[float], List[complex]).
        To produce sufficient statistics for evaluating expectation values,
        circuits have to be run multiple times.
        The results of each repetition are concatenated in OutputRegisters
        (List[List[bool]], List[List[float]], List[List[complex]]).


        Args:
            circuit (Circuit): The circuit that is run on the backend.

        Returns:
            Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.

        Raises:
            TypeError: Circuit argument cannot be converted to qoqo Circuit
            RuntimeError: Running Circuit failed
        """

    def run_measurement_registers(
        self,
        measurement: Union[Cheated, ClassicalRegister, CheatedPauliZProduct, PauliZProduct],
    ) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """
        Run all circuits corresponding to one measurement with the QRyd backend.

        An expectation value measurement in general involves several circuits.
        Each circuit is passed to the backend and executed separately.
        During execution values are written to and read from classical registers
        (List[bool], List[float], List[complex]).
        To produce sufficient statistics for evaluating expectation values,
        circuits have to be run multiple times.
        The results of each repetition are concatenated in OutputRegisters
        (List[List[bool]], List[List[float]], List[List[complex]]).


        Args:
            measurement (Measurement): The measurement that is run on the backend.

        Returns:
            Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.

        Raises:
            TypeError: Circuit argument cannot be converted to qoqo Circuit
            RuntimeError: Running Circuit failed
        """

    def run_measurement(
        self,
        measurement: Union[Cheated, ClassicalRegister, CheatedPauliZProduct, PauliZProduct],
    ) -> Optional[Dict[str, float]]:
        """
        Evaluates expectation values of a measurement with the backend.

        Args:
            measurement (Measurement): The measurement that is run on the backend.

        Returns:
            Optional[Dict[str, float]]: The  dictionary of expectation values.

        Raises:
            TypeError: Measurement evaluate function could not be used
            RuntimeError: Internal error measurement.evaluation returned unknown type
        """

class APIBackend:
    """
    Qoqo backend interfacing QRydDemo WebAPI.

    The WebAPI Backend implements methods available in the QRyd Web API.
    Furthermore, QRyd quantum computer only allows gate operations
    that are available on a device model of a QRyd device (stored in a [crate::QRydDevice]).
    This limitation is introduced by design to check the compatability of quantum programs with a model of the QRyd hardware.
    For simulations of the QRyd quantum computer use the Backend simulator [crate::Backend].

    """

    def __init__(self):
        return

    def post_job(self, quantumprogram) -> str:
        """
        Post to add a new job to be run on the backend and return the location of the job.

        Other free parameters of the job (`seed`, `pcz_theta` etc.)
        are provided by the device given during the initializing of the backend.

        The returned location is the URL of the job in String form
        that can be used to query the job status and result
        or to delete the job.

        Args:
            quantumprogram (qoqo.QuantumProgram): qoqo QuantumProgram to be executed.

        Returns:
            str: URL of the location of the job.
        """

    def get_job_status(self, job_location: str) -> Dict[str, str]:
        """
        Get status of a posted WebAPI job.

        Args:
            job_location (str): location (url) of the job one is interested in.

        Returns:
            Dict[str, str]: status and message of the job.

        """

    def get_job_result(self, job_location: str):
        """
        Get status of a completed WebAPI job.

        Args:
            job_location (str): location (url) of the job one is interested in.

        Returns
            dict: Result of the job.

        """

    def delete_job(self, job_location: str):
        """
        Delete a posted WebAPI job

        Args:
            job_location (str): location (url) of the job one is interested in.

        Raises:
            RuntimeError: Could not delete job.

        """

    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the APIBackend using the bincode crate.

        Returns:
            ByteArray: The serialized APIBackend (in bincode form).

        Raises:
            ValueError: Cannot serialize APIBackend to bytes.
        """

    @staticmethod
    def from_bincode(input: bytearray) -> APIBackend:
        """
        Convert the bincode representation of the APIBackend to a APIBackend using the bincode crate.

        Args:
            input (ByteArray): The serialized APIBackend (in bincode form).

        Returns:
            APIBackend: The deserialized APIBackend.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to APIBackend.
        """

    def to_json(self) -> str:
        """
        Return the json representation of the APIBackend.

        Returns:
            str: The serialized form of APIBackend.

        Raises:
            ValueError: Cannot serialize APIBackend to json.
        """

    @staticmethod
    def from_json(input: str) -> APIBackend:
        """
        Convert the json representation of a APIBackend to a APIBackend.

        Args:
            input (str): The serialized APIBackend in json form.

        Returns:
            APIBackend: The deserialized APIBackend.

        Raises:
            ValueError: Input cannot be deserialized to APIBackend.
        """

    def run_circuit(self, circuit: Circuit) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """
        Run a circuit with the QRyd APIBackend.

        A circuit is passed to the APIBackend and executed.
        During execution values are written to and read from classical registers
        (List[bool], List[float], List[complex]).
        To produce sufficient statistics for evaluating expectation values,
        circuits have to be run multiple times.
        The results of each repetition are concatenated in OutputRegisters
        (List[List[bool]], List[List[float]], List[List[complex]]).


        Args:
            circuit (Circuit): The circuit that is run on the APIBackend.

        Returns:
            Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.

        Raises:
            TypeError: Circuit argument cannot be converted to qoqo Circuit
            RuntimeError: Running Circuit failed
        """

    def run_measurement_registers(
        self,
        measurement: Union[Cheated, ClassicalRegister, CheatedPauliZProduct, PauliZProduct],
    ) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """
        Run all circuits corresponding to one measurement with the QRyd APIBackend.

        An expectation value measurement in general involves several circuits.
        Each circuit is passed to the APIBackend and executed separately.
        During execution values are written to and read from classical registers
        (List[bool], List[float], List[complex]).
        To produce sufficient statistics for evaluating expectation values,
        circuits have to be run multiple times.
        The results of each repetition are concatenated in OutputRegisters
        (List[List[bool]], List[List[float]], List[List[complex]]).


        Args:
            measurement (Measurement): The measurement that is run on the APIBackend.

        Returns:
            Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.

        Raises:
            TypeError: Circuit argument cannot be converted to qoqo Circuit
            RuntimeError: Running Circuit failed
        """

    def run_measurement(
        self,
        measurement: Union[Cheated, ClassicalRegister, CheatedPauliZProduct, PauliZProduct],
    ) -> Optional[Dict[str, float]]:
        """
        Evaluates expectation values of a measurement with the APIBackend.

        Args:
            measurement (Measurement): The measurement that is run on the APIBackend.

        Returns:
            Optional[Dict[str, float]]: The  dictionary of expectation values.

        Raises:
            TypeError: Measurement evaluate function could not be used
            RuntimeError: Internal error measurement.evaluation returned unknown type
        """

    def set_dev(self, dev: bool):
        """
        Setter for the dev option of the APIDevice.

        Args:
            dev (bool): The boolean to set the dev option to.

        """

class qryd_devices:
    """
    Prototype qoqo devices for Rydberg hardware

    .. autosummary::
       :toctree: generated/

       FirstDevice

    """

    def __init__(self):
        return

    def FirstDevice(
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

class api_devices:
    """
    Devices available on the QRydDemo WebAPI.

    .. autosummary::
       :toctree: generated/

       QrydEmuSquareDevice
       QrydEmuTriangularDevice

    """

    def __init__(self):
        return

    def QrydEmuSquareDevice(
        self,
        seed: int,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
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

    def QrydEmuTriangularDevice(
        self,
        seed: int,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
        allow_ccz_gate: Optional[bool],
        allow_ccp_gate: Optional[bool],
    ):
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

class tweezer_devices:
    """
    Tweezer devices for the QRyd platform.

    .. autosummary::
       :toctree: generated/

       TweezerDevice
       TweezerMutableDevice

    """

    def __init__(self):
        return

    def TweezerDevice(
        self,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
        """
        Tweezer Device

        This interface does not allow setting any piece of information about the device
        tweezers. This class is meant to be used by the end user.

        Args:
            controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
                                          It can be hardcoded to a specific value if a float is passed in as String.
            controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
        """

    def TweezerMutableDevice(
        self,
        controlled_z_phase_relation: Optional[Union[str, float]],
        controlled_phase_phase_relation: Optional[Union[str, float]],
    ):
        """
        Tweezer Mutable Device

        This interface allows setting any piece of information about the device
        tweezer.

        Args:
            controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
                                          It can be hardcoded to a specific value if a float is passed in as String.
            controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
        """

class pragma_operations:
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

    def __init__(self):
        return

    def PragmaChangeQRydLayout(self, new_layout: int):
        """
        This PRAGMA operation changes the layout of a QRyd device.

        Before running a circuit a number of layouts can be registered
        in the device with the `add_layout` method.

        This PRAGMA operation switches between the predefined operations.

        Args:
            new_layout (int): The index of the new layout.
        """

    def PragmaShiftQRydQubit(self, new_positions: int, int):
        """
        This PRAGMA operation shifts qubits between tweezer positions.

        The tweezer positions in a FirstQryd device do not all have to be occupied.
        In a partially occupied device the qubits can be shifted between positions inside a row.
        The shift is defined by giving a mapping of qubit number and new row-column positions.

        Args:
            new_positions (Dict[int, (int, int)]): The new positions of the qubits.
        """

    def PragmaDeactivateQRydQubit(self, qubit: int):
        """
        This PRAGMA Operation deactivates a qubit in a QRyd Experimental device.

        In QRyd Experimental devices a quantum state is trapped within an optical tweezer.
        This Operation signals the device to drop the quantum state related to the given qubit.

        Args:
            qubit (int): The qubit to deactivate.
        """

    def PragmaShiftQubitsTweezers(self, shifts: int, int):
        """
        This PRAGMA Operation lists the shift operations to be executed in a QRyd Tweezer device.

        Each tuple contains first the starting tweezer identifier and second the ending tweezer identifier.
        Multiple instances indicate parallel operations.

        Args:
            shifts (list((int, int))): The list of shifts that can run in parallel.
        """

    def PragmaSwitchDeviceLayout(self, new_layout: str):
        """
        This PRAGMA operation changes the layout of a Tweezer device.

        Before running a circuit a number of layouts can be registered
        in the device with the `add_layout` method.

        This PRAGMA operation switches between the predefined operations.

        Args:
            new_layout (str): The name of the new layout.
        """
