# Copyright © 2021-2022 HQS Quantum Simulations GmbH.
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
 
""" Components for the qoqo/roqoqo quantum toolkit by `HQS Quantum Simulations <https://quantumsimulations.de>`_ that support QRyd quantum computers.

The qoqo-qryd/roqoqo-qryd packages provide three components:

* devices: python/rust representation of QRyd devices
* operations: roqoqo Pragma operations specific to QRyd devices that can change the topology of QRyd devices
* simulator (optional): A QuEST based simulator for QRyd devices that checks the availability of the quantum operations on a chosen device during simulation

.. autosummary::
    :toctree: generated/

    qryd_devices
    api_devices
    pragma_operations
    SimulatorBackend
    APIBackend

"""

from .qoqo_qryd import *
devices = qryd_devices