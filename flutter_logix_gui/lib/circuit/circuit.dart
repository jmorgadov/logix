import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';

class Circuit {
  Circuit({
    this.name,
    required this.components,
    required this.componentsPositions,
    required this.connections,
  });

  final String? name;
  final List<Component> components;
  final List<Connection> connections;
  List<Offset> componentsPositions;

  Offset outputPinPosition(int compIdx, int pinIdx) {
    final pinPos = components[compIdx].outputPinRelPosition(pinIdx);
    return componentsPositions[compIdx] + Offset(pinPos.dx, -pinPos.dy);
  }

  Offset inputPinPosition(int compIdx, int pinIdx) {
    final pinPos = components[compIdx].inputPinRelPosition(pinIdx);
    return componentsPositions[compIdx] + Offset(pinPos.dx, -pinPos.dy);
  }

  static Circuit mock() {
    return Circuit(
      name: 'Mock Circuit',
      components: [
        Component(
          name: 'AND',
          type: 'And(2)',
          size: const Size(50, 40),
          inputs: [
            Pin(
              name: 'A',
              position: const Offset(5, 15),
              dir: PinDirection.west,
            ),
            Pin(
              name: 'B',
              position: const Offset(5, 25),
              dir: PinDirection.west,
            ),
          ],
          outputs: [
            Pin(
              name: 'Y',
              position: const Offset(45, 20),
              dir: PinDirection.east,
            ),
          ],
          drawInstructions: [
            DrawInstruction.newBox(
              x1: 5.0,
              y1: 5.0,
              x2: 45.0,
              y2: 35.0,
              color: '#ffffff',
            ),
            DrawInstruction.newText(
              text: 'AND',
              x1: 5.0,
              y1: 5.0,
              x2: 45.0,
              y2: 35.0,
            ),
          ],
        ),
        Component(
          name: 'OR',
          type: 'Or(2)',
          size: const Size(50, 40),
          inputs: [
            Pin(
              name: 'A',
              position: const Offset(5, 15),
              dir: PinDirection.west,
            ),
            Pin(
              name: 'B',
              position: const Offset(5, 25),
              dir: PinDirection.west,
            ),
          ],
          outputs: [
            Pin(
              name: 'Y',
              position: const Offset(45, 20),
              dir: PinDirection.east,
            ),
          ],
          drawInstructions: [
            DrawInstruction.newBox(
              x1: 5.0,
              y1: 5.0,
              x2: 45.0,
              y2: 35.0,
              color: '#ffffff',
            ),
            DrawInstruction.newText(
              text: 'OR',
              x1: 5.0,
              y1: 5.0,
              x2: 45.0,
              y2: 35.0,
            ),
          ],
        ),
      ],
      componentsPositions: [
        const Offset(0, 0),
        const Offset(80, 0),
      ],
      connections: [
        Connection(
          fromCompIdx: 0,
          fromCompOutputIdx: 0,
          toCompIdx: 1,
          toCompInputIdx: 0,
        ),
      ],
    );
  }

  static Circuit fromDescription(
    CircuitDescription circuitDescription,
    CircuitLibrary circuitsLib,
    ComponentLibrary componentsLib,
  ) {
    final components = circuitDescription.components
        .map(
          (e) => Component.fromDescription(
            e,
            componentsLib,
          ),
        )
        .toList();

    final connections = circuitDescription.connections
        .map((e) => Connection.fromDescription(e))
        .toList();

    return Circuit(
      name: circuitDescription.name,
      components: components,
      componentsPositions: circuitDescription.componentsPositions
          .map((e) => Offset(e[0], e[1]))
          .toList(),
      connections: connections,
    );
  }
}

class Component {
  Component({
    required this.name,
    required this.type,
    required this.size,
    required this.inputs,
    required this.outputs,
    required this.drawInstructions,
  });

  final String name;
  final String type;
  final Size size;
  final List<Pin> inputs;
  final List<Pin> outputs;
  final List<DrawInstruction> drawInstructions;

  Offset inputPinRelPosition(int idx) {
    return inputs[idx].position;
  }

  Offset outputPinRelPosition(int idx) {
    return outputs[idx].position;
  }

  static Component fromDescription(
    ComponentDescription componentDescription,
    ComponentLibrary componentsLib,
  ) {
    return Component(
      name: componentDescription.name,
      type: componentDescription.type,
      size: Size(componentDescription.width, componentDescription.height),
      inputs: componentDescription.inputs
              ?.map((e) => Pin.fromDescription(e))
              .toList() ??
          [],
      outputs: componentDescription.outputs
              ?.map((e) => Pin.fromDescription(e))
              .toList() ??
          [],
      drawInstructions: componentDescription.drawInstructions,
    );
  }
}

class Pin {
  Pin({
    required this.name,
    required this.position,
    required this.dir,
  });

  final String name;
  final Offset position;
  final PinDirection dir;

  get isHorizontal => dir == PinDirection.east || dir == PinDirection.west;
  get isVertical => dir == PinDirection.north || dir == PinDirection.south;

  static Pin fromDescription(PinDescription pinDescription) {
    return Pin(
      name: pinDescription.name,
      position: Offset(pinDescription.x, -pinDescription.y),
      dir: pinDescription.direction,
    );
  }
}

class Connection {
  Connection({
    required this.fromCompIdx,
    required this.fromCompOutputIdx,
    required this.toCompIdx,
    required this.toCompInputIdx,
    this.path = const [],
  });

  final int fromCompIdx;
  final int fromCompOutputIdx;
  final int toCompIdx;
  final int toCompInputIdx;
  final List<Offset> path;

  static Connection fromDescription(
      ConnectionDescription connectionDescription) {
    return Connection(
      fromCompIdx: connectionDescription.fromCompIdx,
      fromCompOutputIdx: connectionDescription.fromPin,
      toCompIdx: connectionDescription.toCompIdx,
      toCompInputIdx: connectionDescription.toPin,
      path: connectionDescription.path.map((e) => Offset(e[0], e[1])).toList(),
    );
  }
}
