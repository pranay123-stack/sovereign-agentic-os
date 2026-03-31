from setuptools import setup

setup(
    name="sovereign-agentic-os",
    version="0.1.0",
    description="Official Python client for the Sovereign Agentic OS Agent OS REST API",
    py_modules=["sovereign-agentic-os_sdk", "sovereign-agentic-os_client"],
    python_requires=">=3.8",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
