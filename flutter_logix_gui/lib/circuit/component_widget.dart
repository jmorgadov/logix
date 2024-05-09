import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/pin_widget.dart';
import 'package:flutter_logix_gui/constants.dart';
import 'package:flutter_logix_gui/extensions.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';

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
