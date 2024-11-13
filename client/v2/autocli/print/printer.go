package print

import (
	"encoding/json"
	"io"
	"os"

	"github.com/spf13/cobra"
	"sigs.k8s.io/yaml"
)

const (
	jsonOutput = "json"
	textOutput = "text"
)

// Printer handles formatted output of different types of data
type Printer struct {
	Output       io.Writer
	OutputFormat string
}

// NewPrinter creates a new Printer instance with default stdout
func NewPrinter(cmd *cobra.Command) (*Printer, error) {
	outputFormat, err := cmd.Flags().GetString("output")
	if err != nil {
		return nil, err
	}
	return &Printer{
		Output:       cmd.OutOrStdout(),
		OutputFormat: outputFormat,
	}, nil
}

// PrintString prints the raw string
func (p *Printer) PrintString(str string) error {
	return p.PrintBytes([]byte(str))
}

// PrintRaw prints raw JSON message without marshaling
func (p *Printer) PrintRaw(toPrint json.RawMessage) error {
	return p.PrintBytes(toPrint)
}

// PrintBytes prints and formats bytes
func (p *Printer) PrintBytes(out []byte) error {
	var err error
	if p.OutputFormat == textOutput {
		out, err = yaml.JSONToYAML(out)
		if err != nil {
			return err
		}
	}

	writer := p.Output
	if writer == nil {
		writer = os.Stdout
	}

	_, err = writer.Write(out)
	if err != nil {
		return err
	}

	if p.OutputFormat != textOutput {
		_, err = writer.Write([]byte("\n"))
		if err != nil {
			return err
		}
	}

	return nil
}
