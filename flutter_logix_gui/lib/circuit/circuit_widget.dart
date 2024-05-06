import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/widgets/board/board.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_line.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_widget.dart';

const int kGridSize = 20;
const double kCompPadding = .0;
const double kPinRadius = 4;

class CircuitWidget extends StatefulWidget {
  const CircuitWidget({
    super.key,
    required this.circuit,
  });

  final Circuit circuit;

  @override
  State<CircuitWidget> createState() => _CircuitWidgetState();
}

class _CircuitWidgetState extends State<CircuitWidget> {
  late Circuit _circuit;

  @override
  void initState() {
    super.initState();
    _circuit = widget.circuit;
  }

  @override
  Widget build(BuildContext context) {
    return Board(
      pixelsPerUnit: kGridSize,
      children: [
        for (final connection in _circuit.connections)
          BoardLine(
            color: Colors.grey.shade500,
            strokeWidth: 3,
            points: [
              _circuit.components[connection.fromCompIdx]
                  .outputPinPosition(connection.fromCompOutputIdx),
              _circuit.components[connection.toCompIdx]
                  .inputPinPosition(connection.toCompInputIdx),
            ],
          ),
        for (final component in _circuit.components)
          BoardWidget(
            position: component.position,
            size: component.size,
            onMove: (delta) {
              setState(() {
                component.position += delta;
              });
            },
            child: Listener(
              child: Stack(
                children: [
                  Positioned.fill(
                    child: Padding(
                      padding: const EdgeInsets.all(6.0),
                      child: Container(
                        decoration: BoxDecoration(
                          color: Colors.white,
                          border: Border.all(
                            color: Colors.black,
                            width: 1,
                          ),
                        ),
                      ),
                    ),
                  ),
                  Positioned.fill(
                    child: Center(
                      child: Text(
                        component.name ?? '',
                        style: Theme.of(context).textTheme.labelSmall,
                      ),
                    ),
                  ),
                  for (var i = 0; i < component.inputs; i++)
                    Positioned(
                      left: 0,
                      top: -component.inputPinRelPosition(i).dy - (6 * .4),
                      child: const Pin(
                        direction: PinDirection.west,
                        size: 6,
                      ),
                    ),
                  for (var i = 0; i < component.outputs; i++)
                    Positioned(
                      right: 0,
                      top: -component.outputPinRelPosition(i).dy - (6 * .4),
                      child: const Pin(
                        size: 6,
                        direction: PinDirection.east,
                      ),
                    ),
                ],
              ),
            ),
          ),
      ],
    );
  }
}

enum PinDirection {
  north,
  south,
  east,
  west,
}

class Pin extends StatelessWidget {
  const Pin({
    super.key,
    required this.direction,
    this.size = 4,
  });

  final PinDirection direction;
  final double size;

  get isVertical =>
      direction == PinDirection.north || direction == PinDirection.south;

  get isHorizontal =>
      direction == PinDirection.east || direction == PinDirection.west;

  @override
  Widget build(BuildContext context) {
    final width = isHorizontal ? size : size * .8;
    final height = isVertical ? size : size * .8;
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
              decoration: const BoxDecoration(
                color: Colors.black,
                borderRadius: BorderRadius.all(Radius.circular(4)),
              ),
            ),
          ),
          Positioned(
            left: isVertical
                ? 0
                : direction == PinDirection.west
                    ? size * .5
                    : -size * .5,
            top: isHorizontal
                ? 0
                : direction == PinDirection.north
                    ? size / 2
                    : -size / 2,
            child: Container(
              alignment: isVertical
                  ? direction == PinDirection.north
                      ? Alignment.bottomCenter
                      : Alignment.topCenter
                  : direction == PinDirection.west
                      ? Alignment.centerRight
                      : Alignment.centerLeft,
              width: width,
              height: height,
              decoration: const BoxDecoration(
                color: Colors.black,
                shape: BoxShape.circle,
                border: Border.fromBorderSide(
                  BorderSide(
                    color: Colors.black,
                    width: 1,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
