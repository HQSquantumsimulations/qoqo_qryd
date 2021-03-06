#!/usr/bin/env python3
# -*- coding: utf-8 -*-
#
# Qoqo-qryd documentation build configuration file
#
# This file is execfile()d with the current directory set to its
# containing dir.
#
# Note that not all possible configuration values are present in this
# autogenerated file.
#
# All configuration values have a default; values that are commented out
# serve to show the default.

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#

# sys.path.insert(0, os.path.abspath('../'))

# set __version__
try:
    with open('../roqoqo-qryd/Cargo.toml') as f:
        lines = f.readlines()
    version = lines[-1].strip().split("'")[1].strip()
except Exception:
    version = '0.1.0'
versions = version.split(".")
main_version = "{}.{}".format(versions[0], versions[1])

# -- General configuration ------------------------------------------------

# Sphinx extension module names here, as strings.
extensions = ['sphinx.ext.autodoc',
              'sphinx.ext.doctest',
              'sphinx.ext.todo',
              'sphinx.ext.coverage',
              'sphinx.ext.mathjax',
              'sphinx.ext.viewcode',
              'sphinx.ext.napoleon',
              'sphinx.ext.autosummary',
              'nbsphinx',
              'myst_parser']
# automatically use sphinx-autogen
autosummary_generate = True
autosummary_imported_members = True

# 'both': class and __init__ docstring are concatenated and inserted
# 'class': only class docstring inserted
# 'init': only init docstring inserted
autoclass_content = 'class'
# This value is a list of autodoc directive flags that should be automatically applied to
# all autodoc directives. The supported flags are 'members', 'undoc-members',
# 'private-members', 'special-members', 'inherited-members', 'show-inheritance',
# 'ignore-module-all' and 'exclude-members'.
autodoc_default_flags = ['members', 'exclude-members']
# The default options for autodoc directives.
autodoc_default_options = {
    'members': True,
    'special-members': False,
    'imported-members': False,
    'private-members': False,
    'inherited-members': False,
    'special-members': False,
    'undoc-members': False,
    'exclude-members': '__init__'}
# This value controls the docstrings inheritance. If set to True the docstring for classes
# or methods, if not explicitly set, is inherited form parents.
autodoc_inherit_docstrings = False
# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

# The suffix(es) of source filenames.
source_suffix = {
    '.rst': 'restructuredtext',
    # '.txt': 'markdown',
    '.md': 'markdown',
}

# The master toctree document.
master_doc = 'index'

# General information about the project.
project = 'qoqo-qryd'
copyright = '2022, HQS Quantum Simulations GmbH'
author = 'The qoqo-qryd developers'

# The version info for the project you're documenting, acts as replacement for
# |version| and |release|.
version = main_version
# The full version
release = version

# The language for content autogenerated by Sphinx.
language = 'English'

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# The name of the Pygments (syntax highlighting) style to use.
pygments_style = 'default'


# -- Options for HTML output ----------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
html_theme = "sphinx_rtd_theme"

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory.
html_static_path = []

# Turning off executing notebooks when adding them to Documentation
nbsphinx_execute = 'never'

# Options for latex
latex_engine = 'pdflatex'
latex_documents = [
    (master_doc, 'main.tex', 'qoqo-qryd User Manual',
     'HQS Quantum Simulations GmbH', 'report')
]
