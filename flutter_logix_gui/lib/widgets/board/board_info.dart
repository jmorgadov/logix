import 'package:flutter/material.dart';

class BoardInfo {
  BoardInfo({
    required this.pixelsPerUnit,
    required this.bounds,
    required this.offset,
    required this.scale,
    this.selectionRect,
    this.isDiscrete = false,
  });

  final int pixelsPerUnit;
  final Rect bounds;
  final Offset offset;
  final double scale;
  final Rect? selectionRect;
  final bool isDiscrete;

  get isSelecting => selectionRect != null;

  Offset canvasToBoard(Offset canvasPos, {bool asDiscrete = false}) {
    canvasPos = Offset(
      canvasPos.dx,
      -canvasPos.dy,
    );

    if (asDiscrete && isDiscrete) {
      canvasPos = Offset(
        (canvasPos.dx / pixelsPerUnit).roundToDouble() * pixelsPerUnit,
        (canvasPos.dy / pixelsPerUnit).roundToDouble() * pixelsPerUnit,
      );
    }
    var globalPos = canvasPos * scale + offset;
    return globalPos + Offset(bounds.width / 2, bounds.height / 2);
  }

  Offset localToBoard(Offset localPos) {
    final globalPos = localPos - Offset(bounds.width / 2, bounds.height / 2);
    var pos = globalPos - offset;

    pos = Offset(
      pos.dx / scale,
      pos.dy / scale,
    );

    pos = Offset(
      pos.dx,
      -pos.dy,
    );

    return pos;
  }
}
