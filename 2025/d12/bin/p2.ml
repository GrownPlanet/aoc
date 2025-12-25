let read_input filename = In_channel.(with_open_text filename input_lines)

let rec split_double_nl l_acc ll_acc lines =
  match lines with
  | "" :: r -> split_double_nl [] (List.rev l_acc :: ll_acc) r
  | l :: r -> split_double_nl (l :: l_acc) ll_acc r
  | [] -> List.rev (l_acc :: ll_acc)

let parse_region region =
  Scanf.sscanf region "%dx%d: %[^\n]"
    (fun width height values ->
      let indices =
        String.split_on_char ' ' values |> List.filter_map int_of_string_opt
      in
      (width, height, indices))

let parse_regions regions = List.map parse_region regions

let rec parse_blocks blocks =
  match blocks with
  | [] -> invalid_arg "invalid input"
  | [ e ] -> parse_regions e
  | _ :: r -> parse_blocks r

let parse lines =
  split_double_nl [] [] lines |> parse_blocks

let () =
  read_input "input.txt"
  |> parse
  |> List.map (fun (w, h, r) -> w * h >= 9 * List.fold_left (+) 0 r) (* holy fuck *)
  |> List.fold_left (fun acc b -> acc + if b then 1 else 0) 0
  |> Printf.printf "%d\n"
