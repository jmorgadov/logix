import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_item.dart';

class BoardLine extends BoardItem {
  BoardLine({
    required this.points,
    this.color = Colors.black,
    this.strokeWidth = 2.0,
  }) : super(
          position: _getPosistion(points),
          size: _getSize(points),
        );

  final Color color;
  final double strokeWidth;

  static Offset _getPosistion(List<Offset> points) {
    final minX =
        points.map((point) => point.dx).reduce((a, b) => a < b ? a : b);
    final maxY =
        points.map((point) => point.dy).reduce((a, b) => a > b ? a : b);
    return Offset(minX, maxY);
  }

  static Size _getSize(List<Offset> points) {
    var minX = points[0].dx;
    var minY = points[0].dy;
    final maxX = points.map((point) => point.dx).reduce((a, b) {
      minX = a < b ? a : b;
      return a > b ? a : b;
    });
    final maxY = points.map((point) => point.dy).reduce((a, b) {
      minY = a < b ? a : b;
      return a > b ? a : b;
    });

    return Size(max(maxX - minX, 0.01), max(maxY - minY, 0.01));
  }

  final List<Offset> points;

  @override
  Widget build(BuildContext context, BoardInfo info) {
    final relPoints = points.map((point) {
      return Offset(
        (point.dx - position.dx) / size.width,
        -(point.dy - position.dy) / size.height,
      );
    }).toList();

    return IgnorePointer(
      ignoring: true,
      child: CustomPaint(
        painter: _BoardLinePainter(
          points: relPoints,
          color: color,
          strokeWidth: strokeWidth,
        ),
      ),
    );
  }
}

class _BoardLinePainter extends CustomPainter {
  _BoardLinePainter({
    required this.points,
    this.color = Colors.black,
    this.strokeWidth = 2.0,
  });

  final List<Offset> points;
  final Color color;
  final double strokeWidth;

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..strokeCap = StrokeCap.round;

    for (var i = 0; i < points.length - 1; i++) {
      final p0 = Offset(
        points[i].dx * size.width,
        points[i].dy * size.height,
      );
      final p1 = Offset(
        points[i + 1].dx * size.width,
        points[i + 1].dy * size.height,
      );
      canvas.drawLine(p0, p1, paint);
    }
  }

  @override
  bool shouldRepaint(oldDelegate) {
    return true;
  }
}
