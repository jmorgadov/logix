import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:json_annotation/json_annotation.dart';

part 'circuit_description.g.dart';

typedef CircuitLibrary = Map<String, CircuitDescription>;
typedef ComponentLibrary = Map<String, ComponentDescription>;

@JsonSerializable()
class CircuitDescription {
  final String name;
  final String type;
  final List<ComponentDescription> components;
  final List<List<double>> componentsPositions;
  final List<ConnectionDescription> connections;

  CircuitDescription({
    required this.name,
    required this.type,
    required this.components,
    required this.componentsPositions,
    required this.connections,
  });

  factory CircuitDescription.fromJson(Map<String, dynamic> json) =>
      _$CircuitDescriptionFromJson(json);

  Map<String, dynamic> toJson() => _$CircuitDescriptionToJson(this);
}

@JsonSerializable()
class ComponentDescription {
  final String name;
  final String type;
  final double width;
  final double height;
  final List<PinDescription>? inputs;
  final List<PinDescription>? outputs;
  final List<DrawInstruction> drawInstructions;

  ComponentDescription({
    required this.name,
    required this.type,
    required this.width,
    required this.height,
    required this.inputs,
    required this.outputs,
    required this.drawInstructions,
  });

  factory ComponentDescription.fromJson(Map<String, dynamic> json) =>
      _$ComponentDescriptionFromJson(json);

  Map<String, dynamic> toJson() => _$ComponentDescriptionToJson(this);

  CircuitDescription getCircuitDescription(
    CircuitLibrary circuitLibrary,
  ) {
    final circuit = circuitLibrary[type];
    if (circuit == null) {
      throw Exception('Circuit $type not found in library');
    }
    return circuit;
  }
}

@JsonSerializable()
class ConnectionDescription {
  ConnectionDescription({
    required this.fromCompIdx,
    required this.fromPin,
    required this.toCompIdx,
    required this.toPin,
    required this.path,
  });

  final int fromCompIdx;
  final int fromPin;
  final int toCompIdx;
  final int toPin;
  final List<List<double>> path;

  factory ConnectionDescription.fromJson(Map<String, dynamic> json) =>
      _$ConnectionDescriptionFromJson(json);

  Map<String, dynamic> toJson() => _$ConnectionDescriptionToJson(this);
}

@JsonSerializable()
class PinDescription {
  final String name;
  final PinDirection direction;
  final double x;
  final double y;

  PinDescription({
    required this.name,
    required this.direction,
    required this.x,
    required this.y,
  });

  factory PinDescription.fromJson(Map<String, dynamic> json) =>
      _$PinDescriptionFromJson(json);

  Map<String, dynamic> toJson() => _$PinDescriptionToJson(this);
}

enum DrawInstructionType {
  box,
  text,
  line,
}

@JsonSerializable()
class DrawInstruction {
  final DrawInstructionType type;

  // General
  final String? color;

  // Text
  final String? text;
  final double? fontSize;

  // Line / Box / Text area
  final double? x1;
  final double? y1;
  final double? x2;
  final double? y2;

  final double? lineWidth;
  final String? lineColor;

  const DrawInstruction({
    required this.type,
    this.color,
    this.text,
    this.fontSize,
    this.x1,
    this.y1,
    this.x2,
    this.y2,
    this.lineWidth,
    this.lineColor,
  });

  static DrawInstruction newBox({
    required x1,
    required y1,
    required x2,
    required y2,
    borderColor,
    borderWidth,
    color,
  }) =>
      DrawInstruction(
        type: DrawInstructionType.box,
        color: color,
        x1: x1,
        y1: y1,
        x2: x2,
        y2: y2,
        lineColor: borderColor,
        lineWidth: borderWidth,
      );

  static DrawInstruction newText({
    required text,
    required x1,
    required y1,
    required x2,
    required y2,
    fontSize,
    color,
  }) =>
      DrawInstruction(
        type: DrawInstructionType.text,
        text: text,
        fontSize: fontSize,
        x1: x1,
        y1: y1,
        x2: x2,
        y2: y2,
        color: color,
      );

  static DrawInstruction newLine({
    required x1,
    required y1,
    required x2,
    required y2,
    lineWidth,
    color,
  }) =>
      DrawInstruction(
        type: DrawInstructionType.line,
        x1: x1,
        y1: y1,
        x2: x2,
        y2: y2,
        lineWidth: lineWidth,
        color: color,
      );

  factory DrawInstruction.fromJson(Map<String, dynamic> json) =>
      _$DrawInstructionFromJson(json);

  Map<String, dynamic> toJson() => _$DrawInstructionToJson(this);
}
