"""
This Python script is to generate the Rust documentation for the project.

Make sure the program is run from the root of the project.
"""

import subprocess

def main():
    subprocess.run([
        "cargo", "doc",
        # We don't care about documenting the dependencies
        "--no-deps",
        # Since this isn't a public library, everything will be private
        "--document-private-items",
        # If there's every more than 1 package, we will want to document that too
        "--all",
        # Put it in the root level docs folder
        "--target-dir", "docs",
    ], check=True)

if __name__ == "__main__":
    main()
