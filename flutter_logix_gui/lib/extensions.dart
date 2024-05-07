import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';

extension OffsetExtensions on Offset {
  Offset invY() => Offset(dx, -dy);
  Offset invX() => Offset(-dx, dy);
  Offset sizeScale(Size size) => Offset(dx * size.width, dy * size.height);
}

extension StringExtensions on String {
  Color toColor() {
    final hex = startsWith('#') ? substring(1) : this;
    return Color(int.parse('FF$hex', radix: 16));
  }
}

extension PinDirectionExtensions on PinDirection {
  get isHorizontal => this == PinDirection.east || this == PinDirection.west;
  get isVertical => this == PinDirection.north || this == PinDirection.south;

  get isNorth => this == PinDirection.north;
  get isSouth => this == PinDirection.south;
  get isEast => this == PinDirection.east;
  get isWest => this == PinDirection.west;
}
