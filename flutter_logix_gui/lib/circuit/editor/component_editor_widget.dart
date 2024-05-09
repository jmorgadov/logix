import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';
import 'package:flutter_logix_gui/circuit/component_widget.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:flutter_logix_gui/constants.dart';
import 'package:flutter_logix_gui/extensions.dart';
import 'package:flutter_logix_gui/widgets/board/board.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_widget.dart';

class ComponentEditorWidget extends StatefulWidget {
  const ComponentEditorWidget({
    super.key,
    this.component,
    this.onComponentChanged,
    required this.library,
  });

  final ComponentDescription? component;
  final void Function(ComponentDescription comp)? onComponentChanged;
  final Library library;

  @override
  State<ComponentEditorWidget> createState() => _ComponentEditorWidgetState();
}

class _ComponentEditorWidgetState extends State<ComponentEditorWidget> {
  late ComponentDescription _comp;

  @override
  void initState() {
    super.initState();
    _comp = widget.component ??
        ComponentDescription(
          name: 'New Component',
          type: 'new_component',
          width: 0,
          height: 0,
          inputs: [],
          outputs: [],
          drawInstructions: [],
        );
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: 350,
          child: Align(
            alignment: Alignment.topCenter,
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    _comp.name,
                    style: Theme.of(context).textTheme.headlineMedium,
                  ),
                  Text(
                    _comp.type,
                    style: Theme.of(context).textTheme.bodyLarge,
                  ),
                  const SizedBox(height: 10),
                  SingleChildScrollView(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisAlignment: MainAxisAlignment.start,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        SideBarCategory(
                          title: "Inputs",
                          children: [
                            for (final input in _comp.inputs ?? [])
                              PinEditorItem(
                                pin: input,
                                onDelete: () {
                                  setState(() {
                                    _comp.inputs?.remove(input);
                                  });
                                },
                                onPositionChanged: (x, y) {
                                  setState(() {
                                    input.x = x;
                                    input.y = y;
                                  });
                                },
                              ),
                          ],
                        ),
                        SideBarCategory(
                          title: "Outputs",
                          children: [
                            for (final output in _comp.outputs ?? [])
                              PinEditorItem(
                                pin: output,
                                onDelete: () {
                                  setState(() {
                                    _comp.outputs?.remove(output);
                                  });
                                },
                              ),
                          ],
                        ),
                        SideBarCategory(
                          title: "Draw Instructions",
                          children: [
                            for (final drawInstruction
                                in _comp.drawInstructions)
                              if (drawInstruction.type ==
                                  DrawInstructionType.box)
                                DrawBoxInstEditor(
                                  drawInstruction: drawInstruction,
                                  onDelete: () {
                                    setState(() {
                                      _comp.drawInstructions
                                          .remove(drawInstruction);
                                    });
                                  },
                                  onTopLeftChanged: (x, y) {
                                    setState(() {
                                      drawInstruction.x1 = x;
                                      drawInstruction.y1 = y;
                                    });
                                  },
                                  onBottomRightChanged: (x, y) {
                                    setState(() {
                                      drawInstruction.x2 = x;
                                      drawInstruction.y2 = y;
                                    });
                                  },
                                  onColorChanged: (color) {
                                    setState(() {
                                      drawInstruction.color = color;
                                    });
                                  },
                                  onLineColorChanged: (color) {
                                    setState(() {
                                      drawInstruction.lineColor = color;
                                    });
                                  },
                                ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
        const VerticalDivider(
          width: 1,
        ),
        Expanded(
          child: Board(
            pixelsPerUnit: kGridSize,
            children: [
              BoardWidget(
                position: const Offset(0, 0),
                size: Size(_comp.width, _comp.height),
                child: ComponentWidget(
                  component: Component.fromDescription(
                    _comp,
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

class SideBarCategory extends StatelessWidget {
  const SideBarCategory({
    super.key,
    required this.title,
    required this.children,
    this.onAdd,
  });

  final String title;
  final List<Widget> children;
  final void Function()? onAdd;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Expanded(
              child: Text(
                title,
                style: Theme.of(context).textTheme.headlineSmall,
              ),
            ),
            if (onAdd != null)
              IconButton(
                icon: const Icon(Icons.add),
                onPressed: onAdd,
              ),
          ],
        ),
        ...children,
      ],
    );
  }
}

class PinEditorItem extends StatelessWidget {
  const PinEditorItem({
    super.key,
    required this.pin,
    this.onDelete,
    this.onPositionChanged,
    this.onDirectionChanged,
  });

  final PinDescription pin;
  final void Function()? onDelete;
  final void Function(double x, double y)? onPositionChanged;
  final void Function(PinDirection dir)? onDirectionChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Container(
        padding: const EdgeInsets.all(8),
        decoration: BoxDecoration(
          color: Theme.of(context).primaryColor.withOpacity(.05),
          borderRadius: BorderRadius.circular(5),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  pin.name,
                  style: Theme.of(context).textTheme.bodyLarge!.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.delete),
                  onPressed: onDelete,
                ),
              ],
            ),
            Row(
              children: [
                Text(
                  'Position',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: pin.x.toString(),
                    ),
                    onSubmitted: (value) {
                      onPositionChanged?.call(double.parse(value), pin.y);
                    },
                  ),
                ),
                const SizedBox(width: 10),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: pin.y.toString(),
                    ),
                    onSubmitted: (value) {
                      onPositionChanged?.call(pin.x, double.parse(value));
                    },
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class DrawBoxInstEditor extends StatelessWidget {
  const DrawBoxInstEditor({
    super.key,
    required this.drawInstruction,
    this.onDelete,
    this.onTopLeftChanged,
    this.onBottomRightChanged,
    this.onColorChanged,
    this.onLineColorChanged,
  });

  final DrawInstruction drawInstruction;
  final void Function()? onDelete;
  final void Function(double x, double y)? onTopLeftChanged;
  final void Function(double x, double y)? onBottomRightChanged;
  final void Function(String color)? onColorChanged;
  final void Function(String color)? onLineColorChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Container(
        padding: const EdgeInsets.all(8),
        decoration: BoxDecoration(
          color: Theme.of(context).primaryColor.withOpacity(.05),
          borderRadius: BorderRadius.circular(5),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  'Box',
                  style: Theme.of(context).textTheme.bodyLarge!.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.delete),
                  onPressed: onDelete,
                ),
              ],
            ),
            Row(
              children: [
                Text(
                  'Top Left',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.x1.toString(),
                    ),
                    onSubmitted: (value) {
                      onTopLeftChanged?.call(
                          double.parse(value), drawInstruction.y1!);
                    },
                  ),
                ),
                const SizedBox(width: 10),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.y1.toString(),
                    ),
                    onSubmitted: (value) {
                      onBottomRightChanged?.call(
                          drawInstruction.x1!, double.parse(value));
                    },
                  ),
                ),
              ],
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                Text(
                  'Bottom Right',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.x2.toString(),
                    ),
                    onSubmitted: (value) {
                      onBottomRightChanged?.call(
                          double.parse(value), drawInstruction.y2!);
                    },
                  ),
                ),
                const SizedBox(width: 10),
                SizedBox(
                  width: 100,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.y2.toString(),
                    ),
                    onSubmitted: (value) {
                      onBottomRightChanged?.call(
                          drawInstruction.x2!, double.parse(value));
                    },
                  ),
                ),
              ],
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                Text(
                  'Color',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                SizedBox(
                  width: 210,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.color,
                    ),
                    onSubmitted: (value) {
                      onColorChanged?.call(value);
                    },
                  ),
                ),
              ],
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                Text(
                  'Line Color',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                SizedBox(
                  width: 210,
                  child: TextField(
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(
                      text: drawInstruction.lineColor,
                    ),
                    onSubmitted: (value) {
                      onLineColorChanged?.call(value);
                    },
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
