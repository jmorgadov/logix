import 'dart:math';

import 'package:flutter/material.dart';

class Circuit {
  Circuit({
    this.name,
    required this.components,
    required this.connections,
  });

  final String? name;
  final List<Component> components;
  final List<Connection> connections;

  static Circuit mock() {
    return Circuit(
      components: [
        Component(
          name: 'AND',
          position: const Offset(0, 0),
          inputs: 2,
          outputs: 1,
        ),
        Component(
          name: 'OR',
          position: const Offset(2, 0),
          inputs: 4,
          outputs: 1,
        ),
        Component(
          name: 'NOT',
          position: const Offset(4, 0),
          inputs: 1,
          outputs: 1,
        ),
      ],
      connections: [
        Connection(
          fromCompIdx: 0,
          fromCompOutputIdx: 0,
          toCompIdx: 1,
          toCompInputIdx: 0,
        ),
        Connection(
          fromCompIdx: 1,
          fromCompOutputIdx: 0,
          toCompIdx: 2,
          toCompInputIdx: 0,
        ),
      ],
    );
  }
}

class Component {
  Component({
    this.name,
    required this.position,
    required this.inputs,
    required this.outputs,
  });

  final String? name;
  Offset position;
  final int inputs;
  final int outputs;

  Size get size => Size(1, max(inputs, outputs) / 2);

  Offset inputPinRelPosition(int idx) {
    final divisions = inputs + 1;
    return Offset(
      0,
      -(1 + idx) * size.height / divisions,
    );
  }

  Offset outputPinRelPosition(int idx) {
    final divisions = outputs + 1;
    return Offset(
      size.width,
      -(1 + idx) * size.height / divisions,
    );
  }

  Offset inputPinPosition(int idx) {
    return position + inputPinRelPosition(idx);
  }

  Offset outputPinPosition(int idx) {
    return position + outputPinRelPosition(idx);
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
}
