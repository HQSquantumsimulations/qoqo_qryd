# Copyright © 2021-2025 HQS Quantum Simulations GmbH.
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

import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from qoqo import QuantumProgram
from qoqo_qryd.api_devices import QrydEmuSquareDevice
from qoqo.measurements import ClassicalRegister
from time import sleep

from qoqo_qryd import APIBackend
from typing import Dict, Optional, List
from copy import copy


def test():
    # device = QrydEmuSquareDevice(seed=1)
    # backend = APIBackend(device=device, timeout=20)
    # circuit = Circuit()

    # circuit += ops.RotateX(0, np.pi/2)
    # circuit += ops.RotateX(2, np.pi/2)
    # circuit += ops.RotateX(4, np.pi/2)

    # circuit += ops.DefinitionBit("ro", 6, is_output=True)
    # circuit += ops.PragmaRepeatedMeasurement("ro", 1000, None)

    # measurement = ClassicalRegister(constant_circuit=None, circuits=[circuit])
    # program = QuantumProgram(measurement=measurement, input_parameter_names=[])

    # job_location = backend.post_job(program)

    # for i in range(20):
    #     print(i)
    #     sleep(5)
    #     job_status = backend.get_job_status(job_location)
    #     if job_status["status"] == "completed":
    #         result = backend.get_job_result(job_location)
    #         print(result)
    #         break
    pass


if __name__ == "__main__":
    pytest.main(sys.argv)
