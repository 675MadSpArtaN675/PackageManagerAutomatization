use std::rc::Rc;
use std::cell::RefCell;

use crate::utility::{Stage, PacketManagerResultCode, ErroredPacket};

use std::io::{BufReader, BufRead, Read};
use std::process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Stdio};

pub fn perform_command(basic_command: String,
                secondary_subcommand: String,
                stage: Stage,
                packets: &Vec<String>,
                is_admin: bool,
                error_parser: Option<&mut Box<dyn FnMut(&mut String)>>,
                output_parser: Option<&mut Box<dyn FnMut(&mut String)>>
) -> PacketManagerResultCode
{
    let mut command_parts: Vec<String> = Vec::new();

    if is_admin {
        command_parts.push("sudo".to_string());
    }

    command_parts.extend(basic_command.split(" ").filter(|line| !line.is_empty()).map(|line| line.to_string()));
    command_parts.extend(secondary_subcommand.split(" ").filter(|line| !line.is_empty()).map(|line| line.to_string()));

    let mut _install_command: Command = Command::new(command_parts.remove(0));
    _install_command.stdout(Stdio::piped())
                    .stderr(Stdio::piped());

    for arg in command_parts {
        _install_command.arg(arg);
    }

    _install_command.args(packets);

    let install_process_result = _install_command.spawn();

    if install_process_result.is_err() {
        let error_message = format!("Error: {:?}", install_process_result.unwrap_err());
        return PacketManagerResultCode::Error(error_message, -1, packets.clone())
    }

    let mut install_process: Child = install_process_result.unwrap();

    let _out_stream: Option<ChildStdout> = install_process.stdout.take();
    let _err_stream: Option<ChildStderr> = install_process.stderr.take();

    let mut output_packages: Vec<String> = Vec::new();
    if _out_stream.is_some() {
        output_packages.extend(package_lines_parse(_out_stream.unwrap(), output_parser));
    }
    let _status_code: Result<ExitStatus, std::io::Error> = install_process.wait();
    if _status_code.is_ok() {
        let status_code = _status_code.unwrap();
        let mut errored_packages: Vec<String> = Vec::new();

        if _err_stream.is_some() {
            errored_packages.extend(package_lines_parse(_err_stream.unwrap(), error_parser));
        }

        if status_code.success() && !output_packages.is_empty() {
            return PacketManagerResultCode::Success(None);
        }
        else if status_code.success() {
            return PacketManagerResultCode::Success(Some(output_packages));
        }
        else {
            return PacketManagerResultCode::Error(format!("Error in stage: {}", stage), status_code.code().unwrap(), errored_packages);
        }
    }

    return PacketManagerResultCode::Error(format!("Error of process start. Stage: {}",  stage), -1, vec![])
}

pub fn perform_command_with_standart_parser(
    basic_command: String,
    secondary_subcommand: String,
    stage: Stage,
    packets: &Vec<String>,
    is_admin: bool
) -> PacketManagerResultCode
{
    perform_command(basic_command, secondary_subcommand, stage, packets, is_admin, None, None)
}

fn package_lines_parse<T>(_err_stream: T, parser: Option<&mut Box<dyn FnMut(&mut String)>>) -> Vec<String>
where T: Read {
    let _buf_reader: BufReader<T> = BufReader::new(_err_stream);

    let mut default_function: Box<dyn FnMut(&mut String)> = Box::new(|_line: &mut String| {});
    let parser_func: &mut Box<dyn FnMut(&mut String)> = parser.unwrap_or(&mut default_function);

    let mut _packages_result: Vec<String> = Vec::new();
    for line in _buf_reader.lines() {
        if line.is_ok() {
            let mut line_ref: String = line.unwrap();

            parser_func(&mut line_ref);
            _packages_result.push(line_ref.clone());
        }
    }

    return _packages_result;
}

pub fn catch_err(command_return: PacketManagerResultCode, stage: Stage, errored_packages: &Rc<RefCell<Vec<ErroredPacket>>>) -> PacketManagerResultCode {
    return match command_return {
        PacketManagerResultCode::Error(message, status_code, packets) => {
            for packet in packets.clone() {
                errored_packages.borrow_mut().push(ErroredPacket { name: packet, stage: stage });
            }

            return PacketManagerResultCode::Error(message, status_code, packets);
        },
        PacketManagerResultCode::Success(val) => PacketManagerResultCode::Success(val),
    };
}