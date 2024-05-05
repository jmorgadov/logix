import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';

abstract class BoardItem {
  const BoardItem({
    required this.position,
    required this.size,
    this.margin = EdgeInsets.zero,
  });

  final Offset position;
  final Size size;
  final EdgeInsets margin;

  Widget internalBuild(BuildContext context, BoardInfo info) {
    final localPos = info.canvasToBoardFromInfo(
      position,
    );
    return Positioned(
      left: localPos.dx,
      top: localPos.dy,
      width: size.width * info.pixelsPerUnit,
      height: size.height * info.pixelsPerUnit,
      child: Transform(
        transform: Matrix4.identity()..scale(info.scale),
        child: Padding(
          padding: margin * info.pixelsPerUnit.toDouble(),
          child: build(context, info),
        ),
      ),
    );
  }

  Widget build(BuildContext context, BoardInfo info);
}
