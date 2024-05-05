import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';

abstract class BoardItem {
  const BoardItem({
    required this.position,
    required this.size,
  });

  final Offset position;
  final Size size;

  Widget internalBuild(BuildContext context, BoardInfo info) {
    var localPos = info.canvasToBoard(
      position,
      asDiscrete: true,
    );
    return Positioned(
      left: localPos.dx,
      top: localPos.dy,
      width: size.width,
      height: size.height,
      child: Transform(
        transform: Matrix4.identity()..scale(info.scale),
        child: build(context, info),
      ),
    );
  }

  Widget build(BuildContext context, BoardInfo info);
}
