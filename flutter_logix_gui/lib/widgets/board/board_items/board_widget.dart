import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/widgets/board/board_info.dart';
import 'package:flutter_logix_gui/widgets/board/board_items/board_item.dart';

class BoardWidget extends BoardItem {
  const BoardWidget({
    required super.position,
    required super.size,
    required this.child,
    this.onStartMoving,
    this.onMove,
    this.onEndMoving,
  });

  final Widget child;
  final Function()? onStartMoving;
  final Function(Offset delta)? onMove;
  final Function(Offset position)? onEndMoving;

  @override
  Widget build(BuildContext context, BoardInfo info) {
    return Listener(
      onPointerMove: (details) {
        final delta = Offset(details.delta.dx, -details.delta.dy) / info.scale;
        onMove?.call(delta);
      },
      child: child,
    );
  }
}
