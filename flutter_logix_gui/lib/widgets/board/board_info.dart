import 'package:flutter/material.dart';

class BoardInfo {
  BoardInfo({
    required this.pixelsPerUnit,
    required this.bounds,
    required this.offset,
    required this.scale,
    this.selectionRect,
  });

  final int pixelsPerUnit;
  final Rect bounds;
  final Offset offset;
  final double scale;
  final Rect? selectionRect;

  get isSelecting => selectionRect != null;

  Offset canvasToBoardFromInfo(Offset canvasPos) {
    canvasPos = Offset(
      canvasPos.dx,
      -canvasPos.dy,
    );
    final globalPos = canvasPos * scale * pixelsPerUnit.toDouble() + offset;
    return globalPos + Offset(bounds.width / 2, bounds.height / 2);
  }

  Offset localToBoardFromInfo(Offset localPos) {
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
