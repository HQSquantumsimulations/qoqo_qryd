Welcome to the qoqo-qryd user documentation!
============================================

The qoqo-qryd package provides software to create quantum programs for and run them on the QRyd quantum computer.

The qoqo-qryd package is based on the open source `qoqo <https://github.com/HQSquantumsimulations/qoqo>`_ quantum computing toolkit.


The qoqo-qryd package project provides three components:

   - Backends to run compiled qoqo quantum programs of QRyd hardware or simulators,
   - Device classes representing Qryd quantum hardware and it
   - Specific operations that are only available on QRyd hardware.

To learn more about the general approach to create quantum programs and executing them see :doc:`src/introduction` and :doc:`src/execution`.

:doc:`src/compilation` details the possible compilation steps that can be applied for a quantum program and :doc:`src/qrydspecifics` discusses details of devices and operations available for QRyd.

A collection of example python programs can be found in :doc:`src/examples`.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   src/installation.md
   src/introduction
   src/compilation
   src/execution
   src/qrydspecifics
   src/examples
   src/compiler_api
   src/qoqo_qryd_api


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
