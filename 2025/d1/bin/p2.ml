let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

type rotation = { dir : string; amount : int }

let parse_line line =
  Scanf.sscanf line "%[LR]%d" (fun dir amount -> { dir; amount })

let normalize num =
  let m = 100 in
  let nnum = (m + (num mod m)) mod m in
  let wraps = abs (num - nnum) / m in
  (nnum, wraps)

let rotate position dir amount =
  let new_position, wraps =
    (match dir with
      | "L" -> position - amount
      | "R" -> position + amount
      | _ -> invalid_arg "not left or right")
    |> normalize
  in
  (* only count arriving on 0 as a rotation, this already works when going right *)
  let offset =
    if position = 0 && dir = "L" then -1
    else if new_position = 0 && dir = "L" then 1
    else 0
  in
  (new_position, wraps + offset)

let solve (position, acc) offset =
  let new_position, wraps = rotate position offset.dir offset.amount in
  (new_position, wraps + acc)

let () =
  let _, res =
    read_file "input.txt" |> List.map parse_line |> List.fold_left solve (50, 0)
  in
  Printf.printf "%d\n" res
