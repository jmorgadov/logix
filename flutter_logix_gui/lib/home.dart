import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';
import 'package:flutter_logix_gui/circuit/circuit_widget.dart';
import 'package:flutter_logix_gui/circuit/editor/component_editor_widget.dart';

const int kGridSize = 30;
const double kCompPadding = .4;
const double kPinRadius = 8;

class Home extends StatelessWidget {
  const Home({
    super.key,
    required this.library,
  });

  final Library library;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text('Logix'),
        ),
        body: ComponentEditorWidget(
          component: library.components.values.first,
          library: library,
        )

        //   body: Row(
        //     children: [
        //       Container(
        //         width: 300,
        //         color: Colors.grey[200],
        //         child: const Placeholder(),
        //       ),
        //       Expanded(
        //         child: CircuitBoardWidget(
        //           circuit: Circuit.mock(),
        //         ),
        //       ),
        //     ],
        //   ),
        );
  }
}
