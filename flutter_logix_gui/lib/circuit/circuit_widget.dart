import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/extensions.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:flutter_logix_gui/widgets/board/board.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_line.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_widget.dart';

const int kGridSize = 20;
const double kCompPadding = .0;
const double kPinSize = 5.0;

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

class ComponentWidget extends StatelessWidget {
  const ComponentWidget({
    super.key,
    required this.component,
  });

  final Component component;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        for (final drawInst in component.drawInstructions)
          if (drawInst.type == DrawInstructionType.box)
            Positioned.fill(
              left: drawInst.x1!,
              top: drawInst.y1!,
              right: component.size.width - drawInst.x2!,
              bottom: component.size.height - drawInst.y2!,
              child: Container(
                decoration: BoxDecoration(
                  color: drawInst.color?.toColor() ?? Colors.white,
                  border: Border.all(
                    color: drawInst.lineColor?.toColor() ?? Colors.black,
                    width: drawInst.lineWidth ?? 1,
                  ),
                ),
              ),
            )
          else if (drawInst.type == DrawInstructionType.text)
            Positioned(
              left: drawInst.x1!,
              top: drawInst.y1!,
              right: component.size.width - drawInst.x2!,
              bottom: component.size.height - drawInst.y2!,
              child: Center(
                child: Text(
                  drawInst.text!,
                  style: TextStyle(
                    color: drawInst.color?.toColor() ?? Colors.black,
                    fontSize: drawInst.fontSize ?? 12,
                  ),
                ),
              ),
            ),
        for (final pin in component.inputs)
          Positioned(
            left: pin.position.dx -
                (pin.isVertical
                    ? kPinSize / 2
                    : pin.dir.isWest
                        ? kPinSize
                        : 0),
            top: pin.position.dy -
                (pin.isHorizontal
                    ? kPinSize / 2
                    : pin.dir.isNorth
                        ? kPinSize
                        : 0),
            child: PinWidget(
              direction: pin.dir,
              size: kPinSize,
            ),
          ),
        for (final pin in component.outputs)
          Positioned(
            left: pin.position.dx -
                (pin.isVertical
                    ? kPinSize / 2
                    : pin.dir.isWest
                        ? kPinSize
                        : 0),
            top: pin.position.dy -
                (pin.isHorizontal
                    ? kPinSize / 2
                    : pin.dir.isNorth
                        ? kPinSize
                        : 0),
            child: PinWidget(
              direction: pin.dir,
              size: kPinSize,
            ),
          ),
      ],
    );
  }
}

class PinWidget extends StatelessWidget {
  const PinWidget({
    super.key,
    required this.direction,
    this.size = 4,
  });

  final PinDirection direction;
  final double size;

  get isVertical => direction.isVertical;
  get isHorizontal => direction.isHorizontal;
  get isNorth => direction.isNorth;
  get isSouth => direction.isSouth;
  get isEast => direction.isEast;
  get isWest => direction.isWest;

  @override
  Widget build(BuildContext context) {
    final width = isHorizontal ? size : size;
    final height = isVertical ? size : size;
    return SizedBox(
      width: width,
      height: height,
      child: Stack(
        children: [
          Positioned(
            left: isHorizontal ? 0 : width * .225,
            top: isVertical ? 0 : height * .225,
            child: Container(
              width: isHorizontal ? width : width * .55,
              height: isHorizontal ? height * .55 : height,
              decoration: BoxDecoration(
                color: Colors.black,
                borderRadius: isVertical
                    ? BorderRadius.vertical(
                        top: isNorth ? const Radius.circular(5) : Radius.zero,
                        bottom:
                            isSouth ? const Radius.circular(5) : Radius.zero,
                      )
                    : BorderRadius.horizontal(
                        left: isWest ? const Radius.circular(5) : Radius.zero,
                        right: isEast ? const Radius.circular(5) : Radius.zero,
                      ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
