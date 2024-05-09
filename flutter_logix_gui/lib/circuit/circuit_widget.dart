import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/circuit/component_widget.dart';
import 'package:flutter_logix_gui/constants.dart';
import 'package:flutter_logix_gui/extensions.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:flutter_logix_gui/widgets/board/board.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_line.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_widget.dart';

class CircuitBoardWidget extends StatefulWidget {
  const CircuitBoardWidget({
    super.key,
    required this.circuit,
  });

  final Circuit circuit;

  @override
  State<CircuitBoardWidget> createState() => _CircuitBoardWidgetState();
}

class _CircuitBoardWidgetState extends State<CircuitBoardWidget> {
  late Circuit _circuit;

  @override
  void initState() {
    super.initState();
    _circuit = widget.circuit;
  }

  Pin _getInputPin(int compIdx, int pinIdx) {
    final comp = _circuit.components[compIdx];
    return comp.inputs[pinIdx];
  }

  Pin _getOutputPin(int compIdx, int pinIdx) {
    final comp = _circuit.components[compIdx];
    return comp.outputs[pinIdx];
  }

  Offset _getInputPinEdgePosition(int compIdx, int pinIdx) {
    final pin = _getInputPin(compIdx, pinIdx);
    final pinPos = _circuit.componentsPositions[compIdx] + pin.position.invY();
    final edgeDelta = Offset(
        pin.isVertical
            ? 0
            : pin.dir == PinDirection.east
                ? kPinSize
                : -kPinSize,
        pin.isHorizontal
            ? 0
            : pin.dir == PinDirection.south
                ? kPinSize
                : -kPinSize);
    return pinPos + edgeDelta;
  }

  Offset _getOutputPinEdgePosition(int compIdx, int pinIdx) {
    final pin = _getOutputPin(compIdx, pinIdx);
    final pinPos = _circuit.componentsPositions[compIdx] + pin.position.invY();
    final edgeDelta = Offset(
        pin.isVertical
            ? 0
            : pin.dir == PinDirection.east
                ? kPinSize
                : -kPinSize,
        pin.isHorizontal
            ? 0
            : pin.dir == PinDirection.south
                ? kPinSize
                : -kPinSize);
    return pinPos + edgeDelta;
  }

  @override
  Widget build(BuildContext context) {
    return Board(
      pixelsPerUnit: kGridSize,
      children: [
        for (final connection in _circuit.connections)
          BoardLine(
            color: Colors.grey.shade500,
            strokeWidth: 2.5,
            points: [
              _getOutputPinEdgePosition(
                  connection.fromCompIdx, connection.fromCompOutputIdx),
              ...connection.path,
              _getInputPinEdgePosition(
                  connection.toCompIdx, connection.toCompInputIdx),
            ],
          ),
        for (final entry in _circuit.components.asMap().entries)
          BoardWidget(
            position: _circuit.componentsPositions[entry.key],
            size: entry.value.size,
            onMove: (delta) {
              final idx = entry.key;
              setState(() {
                _circuit.componentsPositions[idx] += delta;
              });
            },
            child: ComponentWidget(
              component: entry.value,
            ),
          ),
      ],
    );
  }
}
