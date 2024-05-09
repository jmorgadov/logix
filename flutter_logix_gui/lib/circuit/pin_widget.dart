import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/pin_direction.dart';
import 'package:flutter_logix_gui/extensions.dart';

class PinWidget extends StatelessWidget {
  const PinWidget({
    super.key,
    required this.direction,
    this.size = 4,
  });

  final PinDirection direction;
  final double size;

  get isVertical => direction.isVertical;
  get isHorizontal => direction.isHorizontal;
  get isNorth => direction.isNorth;
  get isSouth => direction.isSouth;
  get isEast => direction.isEast;
  get isWest => direction.isWest;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: size,
      height: size,
      child: Stack(
        children: [
          Positioned(
            left: isHorizontal ? 0 : size * .225,
            top: isVertical ? 0 : size * .225,
            child: Container(
              width: isHorizontal ? size : size * .55,
              height: isHorizontal ? size * .55 : size,
              decoration: BoxDecoration(
                color: Colors.black,
                borderRadius: isVertical
                    ? BorderRadius.vertical(
                        top: isNorth ? const Radius.circular(5) : Radius.zero,
                        bottom:
                            isSouth ? const Radius.circular(5) : Radius.zero,
                      )
                    : BorderRadius.horizontal(
                        left: isWest ? const Radius.circular(5) : Radius.zero,
                        right: isEast ? const Radius.circular(5) : Radius.zero,
                      ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
