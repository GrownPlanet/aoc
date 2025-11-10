let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse line =
  Scanf.sscanf line "%[UDLR] %d (#%5[0-9a-f]%d)" (fun _ _ code dir ->
      (dir, int_of_string ("0x" ^ code)))

let sign num = if num < 0 then -1 else 1

let calculate_area lines =
  let rec helper lines area_acc line_acc x y =
    match lines with
    | [] -> abs area_acc + (line_acc / 2) + 1
    | line :: rest ->
        let dir, len = parse line in
        let new_x, new_y =
          match dir with
          | 3 -> (x, y - len)
          | 1 -> (x, y + len)
          | 2 -> (x - len, y)
          | 0 -> (x + len, y)
          | _ -> invalid_arg "invalid direction"
        in
        let area = (new_x - x) * y in
        helper rest (area_acc + area) (line_acc + len) new_x new_y
  in
  helper lines 0 0 0 0

let solve () = read_input "input.txt" |> calculate_area |> Printf.printf "%d\n"
