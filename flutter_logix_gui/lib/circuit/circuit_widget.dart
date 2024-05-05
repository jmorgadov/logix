import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/widgets/board/board.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_line.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_widget.dart';

const int kGridSize = 40;
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
            color: Colors.lightGreen,
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
            margin: const EdgeInsets.all(kCompPadding),
            onMove: (delta) {
              setState(() {
                component.position += delta;
              });
            },
            child: Stack(
              children: [
                Positioned.fill(
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
                    left: -1,
                    top: kGridSize * -component.inputPinRelPosition(i).dy -
                        2 -
                        kCompPadding * kGridSize,
                    child: Container(
                      width: 4,
                      height: 4,
                      decoration: const BoxDecoration(
                        color: Colors.black,
                        shape: BoxShape.circle,
                      ),
                    ),
                  ),
                for (var i = 0; i < component.outputs; i++)
                  Positioned(
                    right: -1,
                    top: kGridSize * -component.outputPinRelPosition(i).dy -
                        2 -
                        kCompPadding * kGridSize,
                    child: Container(
                      width: 4,
                      height: 4,
                      decoration: const BoxDecoration(
                        color: Colors.black,
                        shape: BoxShape.circle,
                      ),
                    ),
                  ),
              ],
            ),
          ),
      ],
    );
  }
}
