// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'circuit_description.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CircuitDescription _$CircuitDescriptionFromJson(Map<String, dynamic> json) =>
    CircuitDescription(
      name: json['name'] as String,
      type: json['type'] as String,
      components: (json['components'] as List<dynamic>)
          .map((e) => ComponentDescription.fromJson(e as Map<String, dynamic>))
          .toList(),
      componentsPositions: (json['componentsPositions'] as List<dynamic>)
          .map((e) =>
              (e as List<dynamic>).map((e) => (e as num).toDouble()).toList())
          .toList(),
      connections: (json['connections'] as List<dynamic>)
          .map((e) => ConnectionDescription.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$CircuitDescriptionToJson(CircuitDescription instance) =>
    <String, dynamic>{
      'name': instance.name,
      'type': instance.type,
      'components': instance.components,
      'componentsPositions': instance.componentsPositions,
      'connections': instance.connections,
    };

ComponentDescription _$ComponentDescriptionFromJson(
        Map<String, dynamic> json) =>
    ComponentDescription(
      name: json['name'] as String,
      type: json['type'] as String,
      width: (json['width'] as num).toDouble(),
      height: (json['height'] as num).toDouble(),
      inputs: (json['inputs'] as List<dynamic>?)
          ?.map((e) => PinDescription.fromJson(e as Map<String, dynamic>))
          .toList(),
      outputs: (json['outputs'] as List<dynamic>?)
          ?.map((e) => PinDescription.fromJson(e as Map<String, dynamic>))
          .toList(),
      drawInstructions: (json['drawInstructions'] as List<dynamic>)
          .map((e) => DrawInstruction.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ComponentDescriptionToJson(
        ComponentDescription instance) =>
    <String, dynamic>{
      'name': instance.name,
      'type': instance.type,
      'width': instance.width,
      'height': instance.height,
      'inputs': instance.inputs,
      'outputs': instance.outputs,
      'drawInstructions': instance.drawInstructions,
    };

ConnectionDescription _$ConnectionDescriptionFromJson(
        Map<String, dynamic> json) =>
    ConnectionDescription(
      fromCompIdx: (json['fromCompIdx'] as num).toInt(),
      fromPin: (json['fromPin'] as num).toInt(),
      toCompIdx: (json['toCompIdx'] as num).toInt(),
      toPin: (json['toPin'] as num).toInt(),
      path: (json['path'] as List<dynamic>)
          .map((e) =>
              (e as List<dynamic>).map((e) => (e as num).toDouble()).toList())
          .toList(),
    );

Map<String, dynamic> _$ConnectionDescriptionToJson(
        ConnectionDescription instance) =>
    <String, dynamic>{
      'fromCompIdx': instance.fromCompIdx,
      'fromPin': instance.fromPin,
      'toCompIdx': instance.toCompIdx,
      'toPin': instance.toPin,
      'path': instance.path,
    };

PinDescription _$PinDescriptionFromJson(Map<String, dynamic> json) =>
    PinDescription(
      name: json['name'] as String,
      direction: $enumDecode(_$PinDirectionEnumMap, json['dir']),
      x: (json['x'] as num).toDouble(),
      y: (json['y'] as num).toDouble(),
    );

Map<String, dynamic> _$PinDescriptionToJson(PinDescription instance) =>
    <String, dynamic>{
      'name': instance.name,
      'dir': _$PinDirectionEnumMap[instance.direction]!,
      'x': instance.x,
      'y': instance.y,
    };

const _$PinDirectionEnumMap = {
  PinDirection.north: 'north',
  PinDirection.south: 'south',
  PinDirection.east: 'east',
  PinDirection.west: 'west',
};

DrawInstruction _$DrawInstructionFromJson(Map<String, dynamic> json) =>
    DrawInstruction(
      type: $enumDecode(_$DrawInstructionTypeEnumMap, json['type']),
      color: json['color'] as String?,
      text: json['text'] as String?,
      fontSize: (json['fontSize'] as num?)?.toDouble(),
      x1: (json['x1'] as num?)?.toDouble(),
      y1: (json['y1'] as num?)?.toDouble(),
      x2: (json['x2'] as num?)?.toDouble(),
      y2: (json['y2'] as num?)?.toDouble(),
      lineWidth: (json['lineWidth'] as num?)?.toDouble(),
      lineColor: json['lineColor'] as String?,
    );

Map<String, dynamic> _$DrawInstructionToJson(DrawInstruction instance) =>
    <String, dynamic>{
      'type': _$DrawInstructionTypeEnumMap[instance.type]!,
      'color': instance.color,
      'text': instance.text,
      'fontSize': instance.fontSize,
      'x1': instance.x1,
      'y1': instance.y1,
      'x2': instance.x2,
      'y2': instance.y2,
      'lineWidth': instance.lineWidth,
      'lineColor': instance.lineColor,
    };

const _$DrawInstructionTypeEnumMap = {
  DrawInstructionType.box: 'box',
  DrawInstructionType.text: 'text',
  DrawInstructionType.line: 'line',
};
