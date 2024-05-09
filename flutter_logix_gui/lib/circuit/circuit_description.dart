import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:json_annotation/json_annotation.dart';

part 'circuit_description.g.dart';

typedef CircuitLibrary = Map<String, CircuitDescription>;
typedef ComponentLibrary = Map<String, ComponentDescription>;

class Library {
  CircuitLibrary circuits;
  ComponentLibrary components;

  Library({
    required this.circuits,
    required this.components,
  });

  CircuitDescription? getCircuitDescriptionByComponent(String componentType) {
    final component = components[componentType];
    if (component == null) {
      return null;
    }
    return circuits[component.type];
  }
}

@JsonSerializable()
class CircuitDescription {
  String name;
  String type;
  List<ComponentDescription> components;
  List<List<double>> componentsPositions;
  List<ConnectionDescription> connections;

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
  String name;
  String type;
  double width;
  double height;
  List<PinDescription>? inputs;
  List<PinDescription>? outputs;
  List<DrawInstruction> drawInstructions;

  ComponentDescription({
    required this.name,
    required this.type,
    required this.width,
    required this.height,
    this.inputs,
    this.outputs,
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

  int fromCompIdx;
  int fromPin;
  int toCompIdx;
  int toPin;
  List<List<double>> path;

  factory ConnectionDescription.fromJson(Map<String, dynamic> json) =>
      _$ConnectionDescriptionFromJson(json);

  Map<String, dynamic> toJson() => _$ConnectionDescriptionToJson(this);
}

@JsonSerializable()
class PinDescription {
  String name;
  @JsonKey(name: "dir")
  PinDirection direction;
  double x;
  double y;

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
  DrawInstructionType type;

  // General
  String? color;

  // Text
  String? text;
  double? fontSize;

  // Line / Box / Text area
  double? x1;
  double? y1;
  double? x2;
  double? y2;

  double? lineWidth;
  String? lineColor;

  DrawInstruction({
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
