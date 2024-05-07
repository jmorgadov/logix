import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/circuit/circuit_widget.dart';

const int kGridSize = 30;
const double kCompPadding = .4;
const double kPinRadius = 8;

class TopMenu extends StatelessWidget {
  const TopMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return const Placeholder();
  }
}

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CircuitBoardWidget(
        circuit: Circuit.mock(),
      ),
    );
  }
}
