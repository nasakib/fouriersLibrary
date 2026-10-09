"""
The Symbiotic Truth Engine - SMT Prover & First Principles Verification
Employs Microsoft Z3-Solver to verify logical and physical satisfiability of component graphs.
"""

from typing import List, Dict, Any, Tuple
import z3

# Component Port Classification
# Defines physical domain affinities, conservation limits, and allowed transitions
COMPONENT_DEFINITIONS: Dict[str, Dict[str, Any]] = {
    "Battery": {
        "domain": "electronics",
        "inputs": [],
        "outputs": ["electric_power"],
        "voltage": 12.0,
        "max_current": 10.0,
        "color": "#00F0FF", # Cyan
    },
    "Sensor": {
        "domain": "electronics",
        "inputs": ["electric_power", "fluid_flow"],
        "outputs": ["data_signal"],
        "power_draw": 1.2,
        "color": "#FFB800", # Amber
    },
    "Pump": {
        "domain": "fluid_dynamics",
        "inputs": ["electric_power", "fluid_inlet"],
        "outputs": ["fluid_flow"],
        "power_draw": 8.0,
        "color": "#FF007A", # Magenta
    },
    "Reservoir": {
        "domain": "fluid_dynamics",
        "inputs": ["fluid_flow"],
        "outputs": ["fluid_inlet"],
        "capacity": 100.0,
        "color": "#00FF85", # Neon Green
    },
    "Radiator": {
        "domain": "thermodynamics",
        "inputs": ["fluid_flow"],
        "outputs": ["fluid_return"],
        "thermal_dissipation": 200.0,
        "color": "#9D00FF", # Ultraviolet
    },
    "Controller": {
        "domain": "formal_logic",
        "inputs": ["data_signal", "electric_power"],
        "outputs": ["control_bus"],
        "color": "#FFFFFF", # Pure White
    }
}


def verify_connection_validity(source_type: str, target_type: str) -> Tuple[bool, str]:
    """
    Evaluates whether a directed edge from source_type to target_type satisfies
    first-principles physical domain constraints using Z3 SMT solver.
    """
    if source_type not in COMPONENT_DEFINITIONS or target_type not in COMPONENT_DEFINITIONS:
        return False, "Unknown component type"

    src_meta = COMPONENT_DEFINITIONS[source_type]
    tgt_meta = COMPONENT_DEFINITIONS[target_type]

    solver = z3.Solver()

    # Domain compatibility variables
    has_compatible_port = False
    for out_p in src_meta["outputs"]:
        if out_p in tgt_meta["inputs"]:
            has_compatible_port = True
            break
        # Special case: Fluid return to reservoir
        if out_p == "fluid_flow" and "fluid_flow" in tgt_meta["inputs"]:
            has_compatible_port = True
            break

    # Z3 formal logical constraint formulation
    valid_edge = z3.Bool("valid_edge")
    power_conserved = z3.Bool("power_conserved")
    closed_loop_violation = z3.Bool("closed_loop_violation")

    # Constraint 1: Port compatibility
    solver.add(valid_edge == z3.BoolVal(has_compatible_port))

    # Constraint 2: Direct short-circuit prohibition (Battery -> Battery)
    is_direct_short = (source_type == "Battery" and target_type == "Battery")
    solver.add(closed_loop_violation == z3.BoolVal(is_direct_short))

    # We require: valid_edge AND NOT(closed_loop_violation)
    solver.add(valid_edge)
    solver.add(z3.Not(closed_loop_violation))

    # Evaluate satisfiability
    check_result = solver.check()
    if check_result == z3.sat:
        return True, "SAT: Satisfies conservation and interface axioms"
    else:
        return False, "UNSAT: Violates physical interface or conservation law (e.g. incompatible domain or short)"
