#!/usr/bin/env python3
"""
Shared utilities for OpenAPI code generation fix scripts.
"""

import re
import yaml
from typing import Dict, Any


def convert_to_rust_type_name(name: str) -> str:
    """Convert type names to follow Rust naming conventions.
    Automatically detects and converts acronyms (consecutive uppercase letters) to PascalCase.
    Examples: MCP -> Mcp, HTTP -> Http, URL -> Url, API -> Api, MCPTool -> McpTool, GA -> Ga, etc.
    """
    result = []
    i = 0
    
    while i < len(name):
        # Check if we're at the start of an acronym (2+ consecutive uppercase letters)
        if i < len(name) - 1 and name[i].isupper() and name[i+1].isupper():
            # Found start of acronym, collect all uppercase letters
            acronym_start = i
            while i < len(name) and name[i].isupper():
                i += 1
            
            acronym = name[acronym_start:i]
            
            # Check what comes after the acronym
            if i < len(name) and name[i].islower():
                # Acronym is followed by lowercase (e.g., "MCPtool" or "HTTPServer")
                # The last uppercase letter is actually the start of the next word
                if len(acronym) > 1:
                    # Convert the acronym part (excluding last letter)
                    result.append(acronym[0] + acronym[1:-1].lower())
                    # Back up one position to reprocess the last uppercase letter
                    i -= 1
                else:
                    result.append(acronym)
            else:
                # Acronym is at the end or followed by another capital/nothing
                result.append(acronym[0] + acronym[1:].lower())
        else:
            # Regular character, just append it
            result.append(name[i])
            i += 1
    
    return ''.join(result)


def snake_to_pascal(name: str) -> str:
    """Convert snake_case to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('_'))


def pascal_to_snake(name: str) -> str:
    """Convert PascalCase to snake_case."""
    # Insert underscore before uppercase letters that follow lowercase letters
    s1 = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    # Handle consecutive capitals (e.g., HTTPResponse -> http_response)
    s2 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s1)
    return s2.lower()


def load_spec(spec_path: str) -> Dict[str, Any]:
    """Load OpenAPI specification from YAML file."""
    with open(spec_path, 'r') as f:
        return yaml.safe_load(f)


def save_spec(spec: Dict[str, Any], spec_path: str) -> None:
    """Save OpenAPI specification to YAML file."""
    with open(spec_path, 'w') as f:
        yaml.dump(spec, f, default_flow_style=False, sort_keys=False)


def sanitize_variant_name(name: str) -> str:
    """Sanitize variant name to be valid Rust identifier."""
    # Remove spaces and special characters, convert to PascalCase
    name = name.replace(' ', '_').replace('-', '_').replace(',', '_').replace('.', '_')
    name = name.replace('(', '').replace(')', '').replace('[', '').replace(']', '')
    # Convert to PascalCase
    parts = name.split('_')
    return ''.join(part.capitalize() for part in parts if part)